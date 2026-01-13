# stack (LIFO)

Implementação de uma pilha (Last-In, First-Out).

Esta é a primeira estrutura do repositório e serve como base
para o estilo de código, testes e documentação.

---

## Objetivo

- Entender design de API em Rust
- Trabalhar com ownership simples
- Escrever testes claros
- Estabelecer o padrão do projeto

Nesta primeira versão, vamos usar `Vec<T>` internamente.

---

## API conceitual

- `new`
- `push`
- `pop`
- `peek`
- `len`
- `is_empty`

---

## Invariantes

- Ordem LIFO
- `len` reflete exatamente a quantidade de elementos
- `peek` não remove o topo
- `pop` remove exatamente um elemento

---

## Testes esperados

- stack vazia: `pop` e `peek` retornam `None`
- sequência LIFO correta
- `len` e `is_empty` consistentes
