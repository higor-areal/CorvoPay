# Mercado Pago - Estrutura da Integração

Este documento define a arquitetura oficial de um gateway dentro da CorvoPay.

O objetivo é que **todos os gateways (Mercado Pago, Pagar.me, PagSeguro, Stripe...) possuam exatamente a mesma estrutura**, facilitando manutenção, onboarding de novos desenvolvedores e implementação de novos provedores.

---

# Estrutura

```text
gateways/
└── mercado_pago/
    ├── gateway.rs
    ├── info.rs
    ├── client.rs
    ├── auth.rs
    ├── mod.rs
    │
    ├── pix/
    │   ├── mapper.rs
    │   ├── request.rs
    │   ├── response.rs
    │   ├── service.rs
    │   └── tests.rs
    │
    ├── card/
    └── boleto/
```

---

# gateway.rs

É o ponto de entrada do gateway.

Toda comunicação da aplicação acontece através desta struct.

Ela implementa os contratos do diretório `/contracts`.

Responsabilidades:

* possuir o HTTP Client
* implementar os contratos
* delegar para os serviços específicos
* expor informações do gateway

Nunca deve:

* montar JSON
* conhecer endpoints
* serializar/deserializar
* implementar regras específicas de PIX, cartão ou boleto

Exemplo:

```rust
pub struct MercadoPago {
    client: MercadoPagoClient,
}
```

Implementação do contrato principal:

```rust
impl Gateway for MercadoPago {
    fn info(&self) -> &'static GatewayInfo {
        &INFO
    }
}
```

Implementação de PIX:

```rust
impl PixGateway for MercadoPago {
    async fn create_payment(
        &self,
        input: CreatePaymentIntentInput,
    ) -> Result<CreatePaymentIntentOutput, GatewayError> {

        pix::service::create(
            &self.client,
            input,
        ).await
    }
}
```

Observe que **gateway.rs apenas coordena**.

Toda lógica específica fica dentro de `pix/service.rs`.

---

# info.rs

Contém apenas informações estáticas do gateway.

Nada neste arquivo muda durante a execução.

```rust
pub const INFO: GatewayInfo = GatewayInfo {
    name: GatewayName::MercadoPago,

    capabilities: GatewayCapabilities {
        pix: true,
        card: true,
        boleto: false,
    },

    base_url: "https://api.mercadopago.com",
};
```

Nunca colocar aqui:

* Tokens
* API Keys
* Configuração do produtor
* HTTP Client
* Estado do gateway

---

# client.rs

Responsável exclusivamente pela comunicação HTTP.

Ele não conhece PIX.

Não conhece cartão.

Não conhece boleto.

Ele apenas envia requests.

Exemplo:

```rust
pub struct MercadoPagoClient {
    client: reqwest::Client,
}
```

Funções típicas:

```rust
impl MercadoPagoClient {

    pub async fn post<TReq, TRes>(
        &self,
        endpoint: &str,
        headers: HeaderMap,
        body: &TReq,
    ) -> Result<TRes, GatewayError>
    where
        TReq: Serialize,
        TRes: DeserializeOwned
    {
        todo!()
    }

}
```

Toda lógica HTTP vive aqui.

---

# auth.rs

Responsável pela autenticação.

Exemplos:

* Authorization Header
* Bearer Token
* OAuth
* Assinaturas
* Certificados

Exemplo:

```rust
pub fn authorization_header(
    credentials: &GatewayCredentials,
) -> HeaderValue {

    todo!()
}
```

O restante do gateway nunca deve conhecer como a autenticação funciona.

---

# pix/request.rs

Representa exatamente o JSON esperado pelo Mercado Pago.

Não possui lógica.

```rust
#[derive(Serialize)]
pub struct MpPixRequest {

    pub transaction_amount: Decimal,

    pub payer: MpPayer,

    pub date_of_expiration: String,

}
```

Nenhum campo deve existir apenas para agradar o domínio.

Aqui modelamos exatamente a API do gateway.

---

# pix/response.rs

Representa exatamente a resposta da API.

```rust
#[derive(Deserialize)]
pub struct MpPixResponse {

    pub id: u64,

    pub qr_code: String,

    pub qr_code_base64: String,

    pub status: String,

}
```

---

# pix/mapper.rs

Responsável pelas conversões entre CorvoPay e Mercado Pago.

Nunca faz chamadas HTTP.

Nunca cria requests.

Nunca conhece banco.

Somente conversões.

Entrada:

```rust
impl TryFrom<&CreatePaymentIntentInput>
    for MpPixRequest
{
    type Error = GatewayError;

    fn try_from(...) -> Result<Self, Self::Error> {
        todo!()
    }
}
```

Saída:

```rust
impl TryFrom<MpPixResponse>
    for CreatePaymentIntentOutput
{
    type Error = GatewayError;

    fn try_from(...) -> Result<Self, Self::Error> {
        todo!()
    }
}
```

---

# pix/service.rs

É o cérebro da integração PIX.

Aqui acontece todo o fluxo.

```text
Input

↓

Mapper

↓

HTTP Client

↓

Gateway

↓

HTTP Client

↓

Mapper

↓

Output
```

Exemplo:

```rust
pub async fn create(

    client: &MercadoPagoClient,

    input: CreatePaymentIntentInput,

) -> Result<CreatePaymentIntentOutput, GatewayError> {

    let request = MpPixRequest::try_from(&input)?;

    let response =
        client
            .post(...)
            .await?;

    CreatePaymentIntentOutput::try_from(response)
}
```

Este arquivo conhece:

* mapper
* client
* auth

Ele NÃO conhece:

* banco
* repositories
* router
* failover
* handlers
* use cases

---

# pix/tests.rs

Contém exclusivamente testes da integração PIX.

Exemplos:

* serialização
* desserialização
* mapper
* erros
* respostas inválidas
* mocks da API

Nunca testar Use Cases aqui.

Nunca testar PostgreSQL aqui.

---

# Fluxo completo

```text
CreatePaymentIntentUseCase

        │

        ▼

MercadoPago

(gateway.rs)

        │

        ▼

pix/service.rs

        │

        ▼

mapper.rs

        │

        ▼

client.rs

        │

        ▼

Mercado Pago API

        │

        ▼

client.rs

        │

        ▼

mapper.rs

        │

        ▼

CreatePaymentIntentOutput
```

---

# Responsabilidades

| Arquivo     | Responsabilidade                                                        |
| ----------- | ----------------------------------------------------------------------- |
| gateway.rs  | Ponto de entrada da integração. Implementa contratos e delega chamadas. |
| info.rs     | Dados estáticos do gateway.                                             |
| client.rs   | Comunicação HTTP.                                                       |
| auth.rs     | Autenticação.                                                           |
| request.rs  | Modelos enviados ao gateway.                                            |
| response.rs | Modelos recebidos do gateway.                                           |
| mapper.rs   | Conversão entre CorvoPay ↔ Gateway.                                     |
| service.rs  | Fluxo específico daquela operação.                                      |
| tests.rs    | Testes unitários da integração.                                         |

---

# Regra de ouro

Sempre que surgir uma dúvida sobre onde colocar um código, faça a seguinte pergunta:

> **Esse código existiria se o Mercado Pago deixasse de existir?**

Se a resposta for **sim**, ele provavelmente pertence ao domínio, à aplicação ou à infraestrutura da CorvoPay.

Se a resposta for **não**, ele pertence ao diretório `gateways/mercado_pago`.

Essa regra mantém a arquitetura desacoplada e garante que adicionar um novo gateway seja apenas uma questão de implementar uma nova integração, sem alterar o restante do sistema.
