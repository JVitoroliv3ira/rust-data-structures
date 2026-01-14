# Stack (LIFO)

Implementação de uma **pilha (stack)** em Rust com objetivo **100% educacional**.

Segue o modelo **LIFO (Last-In, First-Out)**: o último elemento a entrar é o
primeiro a sair.

---

## Objetivo

- Entender o comportamento LIFO
- Praticar design de API em Rust
- Trabalhar com ownership simples
- Validar invariantes com testes

---

## Modelo mental

```

top -> [A] [B] [C]

```

- inserção e remoção acontecem no **topo**

---

## API pública

- `new()`
- `push(value: T)`
- `pop() -> Option<T>`
- `peek() -> Option<&T>`
- `len() -> usize`
- `is_empty() -> bool`

---

## Invariantes

- Ordem **LIFO** sempre preservada
- `len()` reflete exatamente a quantidade de elementos
- `peek` não remove o elemento do topo
- `pop` remove exatamente um elemento quando possível

---

## Complexidade

| Operação | Complexidade |
| -------- | ------------ |
| push     | O(1)         |
| pop      | O(1)         |
| peek     | O(1)         |
| len      | O(1)         |
| is_empty | O(1)         |

---

## Implementação

Esta versão utiliza `Vec<T>` internamente.

A estrutura interna é um detalhe; a API é o contrato.

---

## Observação final

Esta implementação prioriza **clareza e aprendizado**, não performance.
