# rust-data-structures

Coleção de estruturas de dados implementadas em Rust com **objetivo 100% educacional**.

Este repositório **não** tem como finalidade substituir, competir ou otimizar
as estruturas da biblioteca padrão (`std`).  
Ele existe exclusivamente para estudo, experimentação e aprendizado profundo.

---

## Objetivo do projeto

Este projeto tem como objetivo **aprender Rust de forma prática e consciente**, explorando:

- ownership e borrowing
- design de APIs seguras
- invariantes de estruturas de dados
- testes

Toda decisão aqui prioriza **clareza, aprendizado e entendimento**, não performance.

---

## Filosofia do projeto

- Educacional acima de tudo
- Uma estrutura por vez
- Implementações simples primeiro
- Cada estrutura possui um README explicando:
  - objetivo
  - API
  - invariantes
  - complexidade
  - testes

---

## Estruturas implementadas

- **Stack (LIFO):** [`crates/stack/README.md`](crates/stack/README.md)
- **Singly Linked List:** [`crates/singly-linked-list/README.md`](crates/singly-linked-list/README.md)

---

## Ordem de implementação (planejada)

- [x] stack (LIFO)
- [x] singly linked list
- [ ] queue (FIFO)
- [ ] binary search tree (BST)

---

## Makefile

O projeto possui um `Makefile` na raiz para facilitar tarefas comuns durante o desenvolvimento.

- `make test` — executa todos os testes do workspace
- `make check` — valida a compilação sem gerar binários
- `make fmt` — formata o código
- `make clippy` — executa lints

O uso do Makefile é apenas um atalho; os comandos `cargo` continuam sendo a fonte de verdade.

---

## Regras do jogo

- Começar sempre com uma API mínima
- Toda estrutura deve ter testes
- Invariantes devem ser documentadas no README da estrutura
- Não usar a `std` como muleta conceitual

---

## Licença

Este projeto é licenciado sob a **GNU General Public License v3.0 (GPL-3.0)**.

Isso significa que:

- o código é livre para uso, estudo e modificação
- qualquer redistribuição deve manter a mesma licença
- o foco é aprendizado aberto e compartilhado

Veja o arquivo `LICENSE` para mais detalhes.
