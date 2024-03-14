use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;

mod model;
use model::cliente::Cliente;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://rafael:2002@localhost/bancorust";

    let pool = MySqlPool::connect(&url).await?;


    loop {
        println!("Escolha uma opção:");
        println!("1. Criar cliente");
        println!("2. Ler cliente");
        println!("3. Atualizar cliente");
        println!("4. Deletar cliente");
        println!("5. Sair");

        let mut opcao1 = String::new();
        std::io::stdin().read_line(&mut opcao1)?;
        let opcao: u32 = match opcao1.trim().parse() {  
            Ok(num)=> num,
            Err(_) => continue,
        };
        match opcao {
            1 => {
                let cliente = Cliente { id:  1, nome: "Nome".to_string() };
                create_cliente(&pool, cliente).await?;
                println!("Cliente criado!");
            },
            2 => {
                let cliente = read_cliente(&pool,  1).await?;
                match cliente {
                    Some(cliente) => println!("{:?}", cliente),
                    None => println!("Cliente não encontrado."),
                }
            },
            3 => {
                let cliente = Cliente { id:  1, nome: "Novo Nome".to_string() };
                update_cliente(&pool, cliente).await?;
                println!("Cliente atualizado!");
            },
            4 => {
                delete_cliente(&pool,  1).await?;
                println!("Cliente deletado!");
            },
            5 => break,
            _ => println!("Opção inválida!"),
        }
    }
    Ok(())
}

async fn create_cliente(pool: &MySqlPool, cliente: Cliente) -> Result<MySqlQueryResult, Error> {
    sqlx::query("INSERT INTO clientes (id, nome) VALUES (?, ?)")
        .bind(cliente.id)
        .bind(cliente.nome)
        .execute(pool)
        .await
}

async fn read_cliente(pool: &MySqlPool, id: i32) -> Result<Option<Cliente>, Error> {
    sqlx::query_as::<_, Cliente>("SELECT id, nome FROM clientes WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

async fn update_cliente(pool: &MySqlPool, cliente: Cliente) -> Result<MySqlQueryResult, Error> {
    sqlx::query("UPDATE clientes SET nome = ? WHERE id = ?")
        .bind(cliente.nome)
        .bind(cliente.id)
        .execute(pool)
        .await
}

async fn delete_cliente(pool: &MySqlPool, id: i32) -> Result<MySqlQueryResult, Error> {
    sqlx::query("DELETE FROM clientes WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
}
