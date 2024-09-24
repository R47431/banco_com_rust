# Gerenciador de Clientes

Este projeto é uma aplicação simples em Rust que utiliza a biblioteca `sqlx` para gerenciar um banco de dados MySQL. O foco principal é a manipulação de registros de clientes, permitindo operações CRUD (Criar, Ler, Atualizar e Deletar) em um banco de dados.

## Tecnologias Utilizadas

- **Rust**: Linguagem de programação utilizada para desenvolver a aplicação.
- **sqlx**: Biblioteca assíncrona para interagir com bancos de dados SQL, aqui utilizada para conectar-se ao MySQL.
- **Tokio**: Runtime assíncrono para Rust, utilizado para permitir a execução de código assíncrono.

## Funcionalidades

A aplicação permite ao usuário:

1. **Criar Cliente**: Adiciona um novo cliente ao banco de dados.
2. **Ler Cliente**: Recupera informações de um cliente existente.
3. **Atualizar Cliente**: Modifica os dados de um cliente já cadastrado.
4. **Deletar Cliente**: Remove um cliente do banco de dados.
5. **Sair**: Encerra a aplicação.

## Como Usar

1. **Configuração do Banco de Dados**: 
   - Certifique-se de que um servidor MySQL esteja em execução e que a tabela `clientes` exista no banco de dados `bancorust`.
   - Ajuste a string de conexão no código, caso necessário.

2. **Execução**:
   - Compile e execute o código. O usuário será apresentado a um menu interativo para realizar as operações desejadas.

## Estrutura do Código

- **Modelo**: O módulo `model` contém a definição da estrutura `Cliente`.
- **Funções Assíncronas**: Cada operação (criação, leitura, atualização e exclusão) é implementada em funções assíncronas que interagem com o banco de dados.

## Contribuição

Sinta-se à vontade para contribuir com melhorias ou correções. Crie um fork deste repositório e envie suas pull requests!
