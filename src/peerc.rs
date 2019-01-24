use runng::*;

fn main() -> Result< (), NngFail >
{
	println!( "PeerC: starting." );

	const URL: &str = "ipc:///tmp/peerAC";

	let factory = Latest::default();
	let pair    = factory.pair_open()?.dial( &URL )?;

	let mut mesg = msg::NngMsg::create()?;

	let text: &'static str = "hello from peerC";

	mesg.append( text.as_ptr() as *const u8, text.len() )?;

	pair.send( mesg )?;

	std::thread::sleep( std::time::Duration::from_millis( 500 ) );

	println!( "PeerC: stopping." );

	Ok(())
}
