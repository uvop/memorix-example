import * as mx from "./schema.generated.mjs";


const main = async () => {
  const memorix = new mx.Memorix();
  await memorix.connect();
  
  await memorix.task.pass_ball.empty(mx.System.NODE);
  const gen = await memorix.task.pass_ball.dequeue(mx.System.NODE);
  
  console.log("Waiting for ball...");
  for await (const ball of gen.asyncIterator) {
    const biggerBall = ball + 1;
    console.log(`Passing the ball with value ${biggerBall}`);
    await memorix.task.pass_ball.enqueue(mx.System.BUN, biggerBall);
  }
}

main().catch(err => {
  console.error(err);
  process.exit(1);
})
