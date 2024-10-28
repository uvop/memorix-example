import * as mx from "./schema.generated.ts";
import { setTimeout } from "node:timers/promises";

const memorix = new mx.Memorix();
await memorix.connect();

console.log("Starting to roll the ball with value 0");
await memorix.task.pass_ball.enqueue(mx.System.NODE, 0);

await memorix.task.pass_ball.empty(mx.System.DENO);
const gen = await memorix.task.pass_ball.dequeue(mx.System.DENO);
await setTimeout(2_000);
for await (const ball of gen.asyncIterator) {
  const biggerBall = ball + 1;
  console.log(`Passing the ball with value ${biggerBall}`);
  await memorix.task.pass_ball.enqueue(mx.System.NODE, biggerBall);
  await setTimeout(2_000);
}
