# trab-u3-edb2

Trabalho da Terceira Unidade da disciplina Estrutura de Dados II

## Uso

### Uso do CRUD

Na pasta `src/stock` estâo as funções que, utilizando a implementação em Rust da árvore B, transformam ela em um CRUD com menu interativo.

- Inicie o programa com:
  ```terminal
  cargo build
  cargo run
  ```

Siga as instruções para interagir com a árvore base (Leitura do arquivo txt disponibilizado pelo professor).
Ao finalizar o programa, sua nova árvore será salva em `out/dot/datas.dot`. Se você tiver o graphviz instalado, pode transformá-la em um png usando o seguinte comando:

```terminal
 dot -Tpng out/dot/datas.dot -o out/png/datas.png
  ```

Assim, a visualização da sua nova árvore será salva em `out/png/datas.png`.

### Testes Árvore B crua

Na pasta `src/btree` está a implementação em Rust da árvore B.

- Teste com:

  ```terminal
  cargo test
  ```

- Gere os gráficos com:

  ```terminal
  cargo run --release
  ```

---
## To-do
- [ ] 1. Implementação Computacional (Teoria)
  - [ ] 1.0. Descreva o ambiente computacional utilizado (Software e Hardware).
  - [ ] 1.1. Faça um algoritmo em forma de fluxograma para a função de exclusão de uma
informação em uma árvore B. (deve ser entregue em formato PDF)
- [ ] 2. Implementação Computacional (Aplicação, Árvore B)
  - [ ] 2.0. Implemente uma árvore B para armazenar e organizar informações de um sistema de controle de estoque. Cada nó da árvore conterá itens de estoque com as seguintes informações: ID, Nome e Quantidade em estoque.
  - [ ] 2.1.Implemente uma árvore B com grau mínimo t. (Considerar as opções t=2 e t=3)
  - [ ] 2.1.Cada nó armazenará até 2t chaves e terá até 2t+1 filhos.
  - [ ] 2.2. Implementar Inserção
  - [ ] 2.2. Implementar Busca
  - [ ] 2.2. Implementar Exclusão
  - [ ] 2.3. Imprima os IDS da árvore por níveis, separando as páginas por colchetes.

&copy; IMD/UFRN 2024
