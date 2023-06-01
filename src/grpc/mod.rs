// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

pub mod server {
    use tonic::{transport::Server, Request, Response, Status};

    use hello_world::greeter_server::{Greeter, GreeterServer};
    use hello_world::{HelloReply, HelloRequest};

    pub mod hello_world {
        tonic::include_proto!("helloworld");
    }

    #[derive(Debug, Default)]
    pub struct MyGreeter {}

    #[tonic::async_trait]
    impl Greeter for MyGreeter {
        async fn say_hello(
            &self,
            request: Request<HelloRequest>,
        ) -> Result<Response<HelloReply>, Status> {
            println!("Got a request: {:?}", request);

            let reply = hello_world::HelloReply {
                message: format!("Hello {}!", request.into_inner().name).into(),
            };

            Ok(Response::new(reply))
        }
    }

    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "[::1]:50051".parse()?;
        let greeter = MyGreeter::default();

        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(addr)
            .await?;

        Ok(())
    }
}

pub mod client {
    use hello_world::greeter_client::GreeterClient;
    use hello_world::HelloRequest;

    pub mod hello_world {
        tonic::include_proto!("helloworld");
    }

    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = GreeterClient::connect("http://[::1]:50051").await?;

        let request = tonic::Request::new(HelloRequest {
            name: "Tonic".into(),
        });

        let response = client.say_hello(request).await?;

        println!("RESPONSE={:?}", response);

        Ok(())
    }
}
