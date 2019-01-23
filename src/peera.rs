use runng::*;


fn main() -> Result< (), NngFail >
{
	println!( "PeerA: starting." );

	const URLB: &str = "ipc:///tmp/peerAB";
	// const URLC: &str = "ipc:///tmp/peerAC";

	let factory = Latest::default();

	let pair_b   = factory.pair_open()?.listen( &URLB )?;
	// let pair_c   = factory.pair_open()?.listen( &URLC )?;

	let mut received_b = pair_b.recv()?;
	// let mut received_c = pair_c.recv()?;

	println!( "PeerA: Received message: {:#?}", std::str::from_utf8( received_b.body() ) );
	// println!( "PeerA: Received message: {:#?}", std::str::from_utf8( received_c.body() ) );

	println!( "PeerA: stopping." );

	Ok(())
}
