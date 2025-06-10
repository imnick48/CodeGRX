import readline from 'readline/promises';
import { stdin as input, stdout as output } from 'process';

async function InputSection(question) {
  const rl = readline.createInterface({ input, output });
  const inp = await rl.question(question+' : ');
  rl.close();
  return inp;
}

export { InputSection };
