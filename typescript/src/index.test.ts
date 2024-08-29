import assert from "node:assert";
import { describe, test } from "node:test";

import { specialPairs } from "./special-pairs";
import { TreeConstructor } from "./tree-constructor";

describe("Algo Test Cases", () => {
  test("special pairs", () => {
    assert.deepEqual(specialPairs(18), [[5, 13], [7, 11]]);
    assert.deepEqual(specialPairs(2), [])
    assert.equal(specialPairs(100).length, 6)
    assert.notEqual(specialPairs(19), []);
  });

  test("tree constructor", () => {
    assert.equal(TreeConstructor(["(1,2)", "(2,4)", "(5,7)", "(7,2)", "(9,5)"]), true);
    assert.equal(TreeConstructor(["(1,2)", "(3,2)", "(2,12)", "(5,2)"]), false);
  });
});
