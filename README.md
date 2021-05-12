# email_validator

## Sobre

Essa é uma API feita em Rust, para um desafio de processo seletivo, que deve verificar se um determinado email
é valido e confiavel, possui 2 métodos de validação, o v1 que utiliza de código próprio (básico) para validar um email,
e o v3 que faz consulta a outra API para determinar se é valido.

O código foi construido usando o framework web [Rocket](https://rocket.rs/master/) na versão 0.5 para uso de async e await.
Foi utilizado o [SQLx](https://github.com/launchbadge/sqlx) para comunicação com o banco Postgres.
E [reqwest](https://github.com/seanmonstar/reqwest) para requisições a [API EVA](https://eva.pingutil.com/)

## TODO's
- Requisitos:
  - [x] Criar servidor com duas rotas de acesso `/` e `/health`.
  - [x] Adicionar rota de validação v1 (código próprio).
  - [x] Adicionar rota de validação v3 (API EVA).
  - [x] Criar banco de dados com as tabelas email_data_v1 e email_data_v3
  - [x] Atualizar rotas para salvar os dados nas tabelas
  - [x] Criar rotas com query string para consultas nas tabelas
  - [x] Suporte a docker
  - [x] Atualizar a rota de validação v3 para funcionar com concorrência, nos casos de um array.
- Extras:
  - [ ] Adicionar mais condições para um email válido na rota de validação v1
  - [ ] Implementar testes unitários para as rotas.
  - [ ] Fazer tratamento de erros e remover os `.unwrap()`.

## Como compilar e executar

### Linux, Windows e MacOS

-   Instale o [PostgreSQL](https://www.postgresql.org/download/)
-   Instale o [Rustup](https://rustup.rs/) (ele gerencia as ferramentas do rust)
-   (Em sistemas Ubuntu, recomendo rodar: `sudo apt install build-essential`)
-   Rode `rustup update nightly`
-   Verifique se os componentes, como `cargo` estão presentes no PATH do seu sistema
-   Rode `cargo install sqlx-cli --no-default-features --features postgres`
-   Clone esse projeto
-   Dentro do diretorio, copie o arquivo .env.example para um novo arquivo .env
    -   Modifique de acordo com a configuração do seu postgres (usuario e senha)
-   Rode `sqlx database create` e `sqlx migrate run`
    -   Isso vai criar o banco no postgres se não existir, e rodar as migrations
-   Se tudo der certo, voce está pronto para rodar: `cargo run` ou `cargo run --release`
    -   O comando vai compilar e executar o binário gerado, o segundo comando irá aplicar o perfil release, e então aplicar as otimizações de código (mais demorado)

### Docker

-   Instale o [docker](https://www.docker.com/get-started) e o [docker-compose](https://docs.docker.com/compose/install/)
-   Clone esse projeto
-   Entre no diretorio do projeto e rode `docker-compose up`
-   Aguarde a criação dos containers e o projeto compilar (essa fase é demorada)
-   Se tudo estiver certo, a api estara rodando em http://localhost:8080/

## Rotas

<table>
<tr>
    <td> URL </td>
    <td> / </td>
</tr>
<tr>
    <td> Metodo </td>
    <td> <strong>GET</strong> </td>
</tr>
<tr>
    <td> Paramêtros da URL </td> <td> n/a  </td>
</tr>
<tr>
<td> Resposta 200 </td>
<td>

```json
{
    "status": "OK",
    "code": 200,
    "results": []
}
```
</td>
</tr>
</table>

<table>
<tr>
    <td> URL </td>
    <td> /health </td>
</tr>
<tr>
    <td> Metodo </td>
    <td> <strong>GET</strong> </td>
</tr>
<tr>
    <td> Paramêtros da URL </td> <td> n/a  </td>
</tr>
<tr>
<td> Resposta 200 </td>
<td>

```json
{
    "status": "OK",
    "code": 200,
    "results": [
        {
            "message": "Servidor executando na porta 8080"
        },
    ]
}
```
</td>
</tr>
</table>

<table>
<tr>
    <td> URL </td>
    <td> /mail/db/v1 </td>
</tr>
<tr>
    <td> Metodo </td>
    <td> <strong>GET</strong> </td>
</tr>
<tr>
    <td> Paramêtros da URL </td>
    <td> Opcionais
<pre>
id=[integer]
email_address=[string]
domain=[string]
valid_syntax=[boolean]
</pre>
    </td>
</tr>
<tr>
    <td> Exemplo </td>
    <td> <pre>/mail/db/v1?email_address=seraphybr@gmail.com&valid_syntax=true</pre> </td>
</tr>
<tr>
<td> Resposta 200 </td>
<td>

```json
{
    "status": "OK",
    "code": 200,
    "results": [
        {
            "id": 1,
            "email_address": "seraphybr@gmail.com",
            "domain": "mail",
            "valid_syntax": true,
            "created_at": "2021-05-12T11:37:01.684178Z"
        },
    ]
}
```
</td>
</tr>
</table>

<table>
<tr>
    <td> URL </td>
    <td> /mail/db/v3 </td>
</tr>
<tr>
    <td> Metodo </td>
    <td> <strong>GET</strong> </td>
</tr>
<tr>
    <td> Paramêtros da URL </td>
    <td> Opcionais
<pre>
id=[integer]
email_address=[string]
domain=[string]
valid_syntax=[boolean]
disposable=[boolean]
webmail=[boolean]
deliverable=[boolean]
catch_all=[boolean]
gibberish=[boolean]
spam=[boolean]
</pre>
    </td>
</tr>
<tr>
    <td> Exemplo </td>
    <td> <pre>/mail/db/v3?email_address=seraphybr@gmail.com&valid_syntax=true</pre> </td>
</tr>
<tr>
<td> Resposta 200 </td>
<td>

```json
{
    "status": "OK",
    "code": 200,
    "results": [
        {
            "email_address": "seraphybr@gmail.com",
            "domain": "gmail.com",
            "valid_syntax": true,
            "disposable": false,
            "webmail": true,
            "deliverable": true,
            "catch_all": false,
            "gibberish": false,
            "spam": false,
            "created_at": "2021-05-12T11:18:11.413685Z"
        }
    ]
}
```
</td>
</tr>
</table>

<table>
<tr>
    <td> URL </td>
    <td> /mail/validation/v1 </td>
</tr>
<tr>
    <td> Metodo </td>
    <td> <strong>POST</strong> </td>
</tr>
<tr>
    <td> Body da Requisição </td>
<td>

```json
{
    "email_address": "seraphybr@gmail.com",
    "domain": "mail"
}
```
</td>
</tr>
<tr>
<td> Resposta 200 </td>
<td>

```json
{
    "status": "OK",
    "code": 200,
    "results": [
        {
            "email_address": "seraphybr@gmail.com",
            "domain": "mail",
            "valid_syntax": true,
        }
    ]
}
```
</td>
</tr>
<tr>
    <td> Observação </td>
    <td> Pode ser tambem enviado um array no body </td>
</tr>
</table>

<table>
<tr>
    <td> URL </td>
    <td> /mail/validation/v3 </td>
</tr>
<tr>
    <td> Metodo </td>
    <td> <strong>POST</strong> </td>
</tr>
<tr>
    <td> Body da Requisição </td>
<td>

```json
{
    "email_address": "seraphybr@gmail.com",
}
```
</td>
</tr>
<tr>
<td> Resposta 200 </td>
<td>

```json
{
    "status": "OK",
    "code": 200,
    "results": [
        {
            "email_address": "seraphybr@gmail.com",
            "domain": "gmail.com",
            "valid_syntax": true,
            "disposable": false,
            "webmail": true,
            "deliverable": true,
            "catch_all": false,
            "gibberish": false,
            "spam": false
        }
    ]
}
```
</td>
</tr>
<tr>
    <td> Observação </td>
    <td>
        Pode ser tambem enviado um array no body. <br/>
        Esse método ira demorar mais, devido a consulta a api Eva.
    </td>
</tr>
</table>
