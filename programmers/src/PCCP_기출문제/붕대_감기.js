function solution(bandage, health, attacks) {
  const [t, x, y] = bandage;
  let currentHealth = health;
  let bandageCount = 0;
  let turn = 0;

  while (attacks.length !== 0) {
    if (currentHealth <= 0) {
      break;
    }

    bandageCount += 1;

    if (turn === attacks[0][0]) {
      const currentAttack = attacks.shift();
      currentHealth -= currentAttack[1];
      bandageCount = 0;
    } else {
      if (bandageCount == t) {
        currentHealth += x + y;
        bandageCount = 0;
      } else {
        currentHealth += x;
      }

      if (currentHealth >= health) {
        currentHealth = health;
      }
    }

    turn += 1;
  }

  return currentHealth <= 0 ? -1 : currentHealth;
}

const bandage = [1, 1, 1];
const health = 5;
const attacks = [
  [1, 2],
  [3, 2],
];

console.log(solution(bandage, health, attacks));
