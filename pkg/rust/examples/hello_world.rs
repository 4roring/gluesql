#[cfg(feature = "sled-storage")]
mod hello_world {
    use {
        gluesql::{
            prelude::{Glue, Payload, Value},
            sled_storage::SledStorage,
        },
        gluesql_core::ast_builder::{table, text, Execute},
        std::fs,
    };

    pub async fn run() {
        /*
            Initiate a connection
        */
        /*
            Open a Sled database, this will create one if one does not yet exist
        */
        let sled_dir = "/tmp/gluesql/hello_world";
        fs::remove_dir_all(sled_dir).unwrap_or(());
        let storage = SledStorage::new(sled_dir).expect("Something went wrong!");
        /*
            Wrap the Sled database with Glue
        */
        let mut glue = Glue::new(storage);

        /*
            Create table then insert a row
        */
        table("greet")
            .create_table()
            .add_column("name TEXT")
            .execute(&mut glue)
            .await
            .expect("Failed Create Table");

        /*
            Insert row
        */
        table("greet")
            .insert()
            .columns("name")
            .values(vec![vec![text("World")]])
            .execute(&mut glue)
            .await
            .expect("Failed Insert Data");

        /*
            Select inserted row
        */
        let result = table("greet")
            .select()
            .project("name")
            .execute(&mut glue)
            .await
            .expect("Failed Select Data");

        /*
            Query results are wrapped into a payload enum, on the basis of the query type
        */
        let rows = match &result {
            Payload::Select { labels: _, rows } => rows,
            _ => panic!("Unexpected result: {:?}", result),
        };

        let first_row = &rows[0];
        let first_value = first_row.iter().next().unwrap();

        /*
            Row values are wrapped into a value enum, on the basis of the result type
        */
        let to_greet = match first_value {
            Value::Str(to_greet) => to_greet,
            value => panic!("Unexpected type: {:?}", value),
        };

        println!("Hello {}!", to_greet); // Will always output "Hello World!"
    }
}

fn main() {
    #[cfg(feature = "sled-storage")]
    futures::executor::block_on(hello_world::run());
}
