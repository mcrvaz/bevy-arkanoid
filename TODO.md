# ECS Arkanoid

### Controles
&nbsp;&nbsp;&nbsp;&nbsp;<kbd>↑</kbd>
<kbd>←</kbd><kbd>↓</kbd><kbd>→</kbd>

&nbsp;&nbsp;&nbsp;&nbsp;<kbd>A</kbd>
<kbd>A</kbd><kbd>S</kbd><kbd>D</kbd>

Jogador movimenta o Vaus (paddle) pelas setas direcionais do teclado ou AWSD.

### Blocos
Cada bloco perde durabilidade ao colidir com uma bola. Quando a durabilidade chega a zero, o bloco é destruído.
Ao destruir blocos, o jogador ganha pontos de acordo com a tabela abaixo:
| Cor      | Pontuação | Durabilidade |
| ----------- | ----------- | ----------- |
| White      | 50       | 1       |
| Blue   | 60        | 1        |
| Green   | 70        | 1        |
| Yellow   | 80        | 1        |
| Orange   | 90        | 2        |
| Pink   | 100        | 2        |
| Red   | 110        | 3        |
| Gold   | -        | -        |

Adicionalmente, quando um bloco é destruído, há uma chance de 50% de gerar um powerup qualquer.

### Powerups
Quando uma bola colide com um powerup, o jogador recebe um bônus de acordo com a tabela abaixo:
| Cor      | Bônus |
| ----------- | ----------- |
| Orange   | Reduz a velocidade de todas as bolas |
| Blue   | Duplica a quantidade atual de bolas |
| Green   | Acelera todas as bolas |
| Pink   | Duplica a quantidade atual de Vaus |
| Red   | Duplica a quantidade atual de vidas |

### Condição de vitória/derrota
Quando o jogador não possuir mais vidas e uma bola colidir com o gol, o jogo é finalizado na condição de derrota.
Quando não houver mais nenhum bloco visível na tela, o jogo é finalizado na condição de vitória.

## Tarefas
- [ ] Implementar movimentação do Vaus
- [ ] Reduzir quantidade de vidas na colisão da bola com o gol
- [ ] Adicionar pontuação ao quebrar um bloco
- [ ] Exibir pontuação atual e high score
- [ ] Exibir quantidade de vidas restantes
- [ ] Implementar Powerups
    - [ ] **Orange** - Reduz a velocidade de todas as bolas
    - [ ] **Blue** - Duplica a quantidade atual de bolas
    - [ ] **Green** - Acelera todas as bolas
    - [ ] **Pink** - Duplica a quantidade atual de Vaus
    - [ ] **Red** - Duplica a quantidade atual de vidas
- [ ] Implementar condições de vitória e derrota
- [ ] Armazenar high score ao fim da partida
- [ ] Recomeçar partida
- [ ] Animar a destruição dos blocos