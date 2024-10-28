import * as mx from "./schema.generated.ts";

const memorix = new mx.Memorix();
await memorix.connect();

await memorix.task.pass_ball.empty(mx.System.BUN);
const gen = await memorix.task.pass_ball.dequeue(mx.System.BUN);

console.log("Waiting for ball...");
for await (const ball of gen.asyncIterator) {
  const biggerBall = ball + 1;
  console.log(`Passing the ball with value ${biggerBall}`);
  await memorix.task.pass_ball.enqueue(mx.System.RUST, biggerBall);
}
