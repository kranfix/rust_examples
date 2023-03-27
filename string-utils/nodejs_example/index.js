//import { bindings } from "@kranfix/string-utils";
const { bindings } = require("@kranfix/string-utils")

async function main() {
  const su = await bindings.string_utils();
  
  const texts = ["", "hello"];

  for (const s of texts) {
    console.log(`"${s}" is empty? ${su.isEmpty(s)}`);
  }
}

main();
