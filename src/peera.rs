use runng::*;


fn main() -> Result< (), NngFail >
{
	println!( "PeerA: starting." );

	const URLB: &str = "ipc:///tmp/peerAB";
	const URLC: &str = "ipc:///tmp/peerAC";

	let factory = Latest::default();

	let pairB   = factory.pair_open()?.listen( &URLB )?;
	let pairC   = factory.pair_open()?.listen( &URLC )?;

	let mut receivedB = pairB.recv()?;
	let mut receivedC = pairC.recv()?;

	println!( "PeerA: Received message: {:#?}", std::str::from_utf8( receivedB.body() ) );
	println!( "PeerA: Received message: {:#?}", std::str::from_utf8( receivedC.body() ) );

	println!( "PeerA: stopping." );

	Ok(())
}
