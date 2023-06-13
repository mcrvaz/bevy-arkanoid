# ECS Arkanoid

## Gameplay
Destrua blocos para ganhar mais pontos.

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
| Grey      | 50       | 1       |
| White   | 50        | 1        |
| Orange   | 60        | 1        |
| Light Blue   | 70        | 1        |
| Green   | 80        | 2        |
| Red   | 90        | 2        |
| Dark Blue   | 100        | 2        |
| Light Orange   | 110        | 3        |
| Yellow   | 120        | 4        |
| Gold   | -        | -        |

### Powerups
Quando uma bola colide com um powerup, o jogador recebe um bônus de acordo com a tabela abaixo:
| Cor      | Bônus |
| ----------- | ----------- |
| A   |  Reduz a velocidade de todas as bolas|
| B   | Duplica a quantidade atual de bolas|
| C   |    Acelera todas as bolas|
| D   | Duplica a quantidade atual de Vaus|
| E   | Duplica a quantidade atual de vidas|

### Inimigos

### Pontuação
Ao final de cada round a pontuação do jogador deve ser comparada com a maior pontuação anterior e substituí-la caso seja maior.
O valor da maior pontuação deve persistir entre as partidas.

### Condição de vitória

## Tarefas