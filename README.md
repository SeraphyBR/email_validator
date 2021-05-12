# email_validator

## Sobre

-- todo

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
