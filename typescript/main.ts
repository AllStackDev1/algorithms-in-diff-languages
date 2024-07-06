import { specialPairs } from "./src/special-pairs";
import { TreeConstructor } from "./src/tree-constructor";

(() => {
  console.log("Special Pairs - TS", specialPairs(18));
  console.log("Tree Constructor - TS", TreeConstructor(["(1,2)", "(2,4)", "(5,7)", "(7,2)", "(9,5)"]));
  console.log("Tree Constructor - TS", TreeConstructor(["(1,2)", "(3,2)", "(2,12)", "(5,2)"]));
})();
