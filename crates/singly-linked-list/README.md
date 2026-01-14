# Singly Linked List

Implementação de uma **lista encadeada simples (singly linked list)** em Rust com objetivo
**100% educacional**.

Implementada manualmente (sem `Vec`), com foco em ownership, borrowing e memória.

---

## Objetivo

- Entender encadeamento de nós e ownership (`Box`)
- Praticar uma API segura e simples
- Manter invariantes claras e testadas

---

## Modelo mental

```

head -> [ value | next ] -> [ value | next ] -> None

```

A lista possui apenas o `head`. Cada nó é dono do próximo nó.

---

## API pública

- `new()`
- `push_front(value: T)`
- `pop_front() -> Option<T>`
- `peek_front() -> Option<&T>`
- `len() -> usize`
- `is_empty() -> bool`
- `clear()`

---

## Invariantes

- `len() == 0` ⇔ lista vazia (`head == None`)
- `push_front` incrementa `len`
- `pop_front` decrementa `len` apenas quando remove um elemento
- `peek_front` não altera o estado
- sem ciclos: cada nó aponta para no máximo um próximo nó

---

## Complexidade

| Operação   | Complexidade |
| ---------- | ------------ |
| push_front | O(1)         |
| pop_front  | O(1)         |
| peek_front | O(1)         |
| len        | O(1)         |
| is_empty   | O(1)         |
| clear      | O(n)         |

---

## Implementação

Esta versão é baseada em nós encadeados com `Option<Box<Node<T>>>`.

A implementação interna é um detalhe; a API é o contrato.

---

## Observação final

Esta implementação prioriza **clareza e aprendizado**, não performance.
