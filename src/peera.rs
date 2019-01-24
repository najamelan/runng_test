/// Simple test case for runng
/// Problems:
/// 1. mesg.append takes a *const u8
/// 2. NngFail is not compatible with failure::Error
/// 3. protocol::pair1::Pair1 does not implement Debug, and probably not all the other usefull derived stuff (https://doc.rust-lang.org/book/appendix-03-derivable-traits.html?highlight=derivable,traits#appendix-c-derivable-traits)
/// 4. There is no connection close? pair.dial, but not pair.close?

use runng::*;
use actix::{ Actor, Arbiter, SyncArbiter };
use futures::future::Future;

fn main() -> Result< (), NngFail >
{
	println!( "PeerA: starting." );

	let system = actix::System::new( "Ekke_Actix_System" );

	let processor = SyncArbiter::start( 1, move || Processor{} );

	let pb = Peer::new( processor.clone(), "ipc:///tmp/peerAB" )?;
	let pc = Peer::new( processor.clone(), "ipc:///tmp/peerAC" )?;

	let peer_b = SyncArbiter::start( 1, move || pb.clone() );
	let peer_c = SyncArbiter::start( 1, move || pc.clone() );

	let run_b  = peer_b.send( Run{} );
	let run_c  = peer_c.send( Run{} );

	Arbiter::spawn
	(
		run_b.map( |res|
		{
			match res
			{
				Ok (_) => println! ( "The PeerB was successfully started." ),
				Err(_) => eprintln!( "An Error occurred while trying to start PeerB." ),
			}
		})

			.map_err( |_| ())
	);

	Arbiter::spawn
	(
		run_c.map( |res|
		{
			match res
			{
				Ok (_) => println! ( "The PeerC was successfully started." ),
				Err(_) => eprintln!( "An Error occurred while trying to start PeerB." ),
			}
		})

			.map_err( |_| ())
	);



	// let mut received_c = pair_c.recv()?;

	//println!( "PeerA: Received message: {:#?}", std::str::from_utf8( received_b.body() ) );
	// println!( "PeerA: Received message: {:#?}", std::str::from_utf8( received_c.body() ) );

	system.run();

	println!( "PeerA: stopping." );

	Ok(())
}





pub struct Processor {}

impl Actor for Processor
{
	type Context = actix::SyncContext< Self >;
}


#[ derive( Debug ) ]
//
pub struct StringMessage
{
	body: String,
}


impl actix::Message for StringMessage
{
	type Result = Result< (), failure::Error >;
}


impl actix::Handler< StringMessage > for Processor
{
	type Result = Result< (), failure::Error >;

	fn handle( &mut self, msg: StringMessage, _: &mut Self::Context ) -> Self::Result
	{
		println!( "Received message: {:?}", &msg );

		Ok(())
	}
}




#[ derive( Clone )]
//
pub struct Peer
{
	processor: actix::Addr< Processor >,
	uri      : String                  ,
	pair     : protocol::pair1::Pair1  ,

}


impl Peer
{
	pub fn new( processor: actix::Addr< Processor >, url: &str ) -> Result< Self, result::NngFail >
	{
		let pair   = Latest::default().pair_open()?.listen( url )?;
		let uri    = url.to_string();

		Ok
		(
			Self
			{
				processor ,
				uri       ,
				pair      ,
			}
		)
	}
}


impl Actor for Peer
{
	type Context = actix::SyncContext< Self >;
}


#[ derive( Debug ) ]
//
pub struct Run {}


impl actix::Message for Run
{
	type Result = Result< (), NngFail >;
}


impl actix::Handler< Run > for Peer
{
	type Result = Result< (), NngFail >;

	fn handle( &mut self, _: Run, _: &mut Self::Context ) -> Self::Result
	{
		println!( "Start listening for messages on: {:?}", self.uri );

		loop
		{
			let mut recv = self.pair.recv()?;

			println!( "peerA: We received a message." );

			let string   = std::str::from_utf8( recv.body() ).expect( "A valid string" ).to_string();

			// send the message to the processor
			//
			let res = self.processor.send( StringMessage{ body: string } );

			Arbiter::spawn
			(
				res.map( |res|
				{
					match res
					{
						Ok (_) => println! ( "peerA: Successfully forwarded message to processor." ),
						Err(_) => eprintln!( "An Error occurred while trying to forward message to processor." ),
					}
				})

					.map_err( |_| ())
			);
		}

		// Ok(())
	}
}


