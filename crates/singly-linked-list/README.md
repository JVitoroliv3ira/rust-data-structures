# Singly Linked List

Implementação de uma **lista encadeada simples (singly linked list)** com objetivo
**100% educacional**.

Esta estrutura é implementada manualmente, sem uso de `Vec`, com foco em entender
ownership, borrowing e organização de memória em Rust.

---

## Objetivo

- Aprender como dados encadeados funcionam internamente
- Praticar ownership entre nós (`Box`)
- Entender como mover e emprestar dados com segurança
- Implementar invariantes e validá-las com testes
- Servir de base para outras estruturas (ex: stack baseada em lista)

---

## Modelo mental

Cada nó:

- possui um valor
- possui (ou não) o próximo nó

```

head -> [ value | next ] -> [ value | next ] -> None

```

A lista possui **apenas o head**.  
Cada nó é dono do próximo nó.

---

## API pública

A lista expõe apenas operações sobre **valores**, nunca sobre nós.

- `new`
- `push_front`
- `pop_front`
- `peek_front`
- `len`
- `is_empty`
- `clear`

Os detalhes de `Node`, `Box` e encadeamento são **internos**.

---

## Operações

### `push_front`

- Insere um novo elemento no início da lista
- O novo elemento se torna o `head`
- Complexidade: **O(1)**

### `pop_front`

- Remove o elemento do início da lista
- Retorna o valor removido
- Complexidade: **O(1)**

### `peek_front`

- Retorna uma referência ao valor do `head`
- Não remove o elemento
- Complexidade: **O(1)**

---

## Invariantes

- `head == None` ⇔ lista vazia
- `len == 0` ⇔ lista vazia
- `push_front` incrementa `len`
- `pop_front` decrementa `len` apenas quando remove um elemento
- A lista nunca contém ciclos
- O `head` sempre aponta para o primeiro elemento lógico da lista

---

## Complexidade

| Operação   | Complexidade |
| ---------- | ------------ |
| push_front | O(1)         |
| pop_front  | O(1)         |
| peek_front | O(1)         |
| len        | O(1)         |
| clear      | O(n) (drop)  |

---

## Testes

Os testes validam:

- estado inicial da lista
- comportamento em lista vazia
- manutenção correta de `len` e `is_empty`
- ordem correta dos elementos
- alternância de operações
- reset completo via `clear`

Os testes **não acessam o `head` diretamente**, validando apenas a API pública.

---

## Decisões de design

- A lista é otimizada para operações no **front**
- Não há `push_back` ou `pop_back` nesta versão
- Um contador interno (`len`) é mantido para acesso O(1)
- O nó (`Node`) é um detalhe de implementação e não é exposto

---

## Evoluções possíveis

- Tornar a lista genérica (`SinglyLinkedList<T>`)
- Adicionar `peek_front_mut`
- Implementar iteradores (`iter`, `iter_mut`, `into_iter`)
- Implementar uma `Stack` reutilizando esta lista
- Comparar esta implementação com uma baseada em `Vec`

---

## Observação final

Esta implementação prioriza **clareza e aprendizado** sobre performance ou
completude.  
O objetivo é entender _por que_ a estrutura funciona — não apenas _fazer funcionar_.
