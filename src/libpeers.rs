use actix::prelude::*;

struct Socket
{
	uri: String
}


struct Peer
{

}

impl Actor for Peer
{
	type Context = Context< Self >;
}
