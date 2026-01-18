# HashMap

Implementação de um **hash map** em Rust com objetivo **100% educacional**.

Armazena pares **chave → valor** usando **tabela hash com encadeamento (chaining)**:
cada bucket é uma lista (`Vec`) de entradas que compartilham o mesmo índice.

---

## Objetivo

- Entender como funciona um hash map por baixo dos panos
- Praticar hashing, colisões e rehash
- Exercitar design de API segura em Rust
- Validar invariantes com testes

---

## Modelo mental

```

index
0  -> []
1  -> [(K1, V1)]
2  -> [(K2, V2), (K3, V3)]
3  -> []
...

```

- a chave é transformada em um número (hash)
- o índice é `hash % capacity`
- colisões são resolvidas via **encadeamento**
- buckets crescem dinamicamente
- a tabela cresce via **resize + rehash**

---

## API pública

O hash map é genérico (`HashMap<K, V>`), com restrição: `K: Eq + Hash`

- `new()`
- `insert(key: K, value: V)`
- `get(key: K) -> Option<&V>`
- `remove(key: K) -> Option<V>`
- `contains(key: K) -> bool`
- `len() -> usize`
- `is_empty() -> bool`
- `clear()`

Também implementa `Default`.

---

## Invariantes

- Cada chave é única
- Inserir uma chave existente **substitui o valor**
- `len()` representa o número real de pares armazenados
- `len() == 0` ⇔ mapa vazio
- Mapa vazio:
  - `get(...) == None`
  - `remove(...) == None`
- `get` não transfere ownership
- `clear` remove todos os pares
- `capacity` nunca é zero
- `capacity == buckets.len()`

---

## Resize / Rehash

O hash map cresce automaticamente quando o **load factor** passa de ~0.75.

```

load_factor = len / capacity

```

Quando o resize acontece:

1. a capacidade dobra
2. novos buckets são criados
3. todas as chaves são **rehashadas**
4. os pares são redistribuídos

O resize é transparente para quem usa a API.

---

## Complexidade

| Operação | Complexidade média |
| -------- | ------------------ |
| insert   | O(1)               |
| get      | O(1)               |
| remove   | O(1)               |
| contains | O(1)               |
| len      | O(1)               |
| is_empty | O(1)               |
| clear    | O(n)               |
| resize   | O(n)               |

> No pior caso (muitas colisões), operações podem degradar para O(n).

---

## Implementação interna

- `Vec<Vec<(K, V)>>` como tabela de buckets
- hashing via `DefaultHasher`
- encadeamento para colisões
- resize com rehash completo
- nenhum método expõe a estrutura interna

---

## Observação final

Esta implementação **não substitui** `std::collections::HashMap`.

Ela existe para:

- aprendizado
- leitura
- entendimento profundo de hashing e ownership
