# email_validator

## Sobre
-- todo

## Rotas
-- todo
## Como compilar e executar
### Linux, Windows e MacOS
- Instale o [PostgreSQL](https://www.postgresql.org/download/)
- Instale o [Rustup](https://rustup.rs/) (ele gerencia as ferramentas do rust)
- (Em sistemas Ubuntu, recomendo rodar: `sudo apt install build-essential`)
- Rode `rustup update nightly`
- Verifique se os componentes, como `cargo` estão presentes no PATH do seu sistema
- Rode `cargo install sqlx-cli`
- Clone esse projeto
- Dentro do diretorio, copie o arquivo .env.example para um novo arquivo .env
  - Modifique de acordo com a configuração do seu postgres (usuario e senha)
- Rode `sqlx database create` e `sqlx migrate run`
  - Isso vai criar o banco no postgres se não existir, e rodar as migrations
- Se tudo der certo, voce está pronto para rodar: `cargo run` ou `cargo run --release`
  - O comando vai compilar e executar o binário gerado, o segundo comando irá aplicar o perfil release, e então aplicar as otimizações de código (mais demorado)

### Docker
- Instale o [docker](https://www.docker.com/get-started) e o [docker-compose](https://docs.docker.com/compose/install/)
- Clone esse projeto
- Entre no diretorio do projeto e rode `docker-compose up`
- Aguarde a criação dos containers e o projeto compilar (essa fase é demorada)
- Se tudo estiver certo, a api estara rodando em http://localhost:8080/