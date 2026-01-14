# Queue (FIFO)

Implementação de uma **fila (queue)** em Rust com objetivo **100% educacional**.

Segue o modelo **FIFO (First-In, First-Out)**: o primeiro elemento a entrar é o
primeiro a sair.

---

## Objetivo

- Entender o comportamento FIFO
- Praticar design de API segura em Rust
- Validar invariantes com testes

---

## Modelo mental

```

front -> [A] [B] [C] <- back

```

- inserção no **back**
- remoção no **front**

---

## API pública

A queue é genérica (`Queue<T>`):

- `new()`
- `enqueue(value: T)`
- `dequeue() -> Option<T>`
- `peek_front() -> Option<&T>`
- `len() -> usize`
- `is_empty() -> bool`
- `clear()`

Também implementa `Default`.

---

## Invariantes

- FIFO sempre preservado
- `len() == 0` ⇔ fila vazia
- fila vazia:
  - `peek_front() == None`
  - `dequeue() == None`
- `peek_front` não altera o estado
- `clear` reseta completamente a fila

---

## Complexidade

| Operação   | Complexidade |
| ---------- | ------------ |
| enqueue    | O(1)         |
| dequeue    | O(1)         |
| peek_front | O(1)         |
| len        | O(1)         |
| is_empty   | O(1)         |
| clear      | O(n)         |

---

## Implementação

Esta versão usa `VecDeque<T>` internamente.  
A estrutura interna é um detalhe; a API é o contrato.

---

## Observação final

Esta implementação prioriza **clareza e aprendizado**, não performance.
