import unittest

from special_pairs import special_pairs
from tree_constructor import tree_constructor

class RunTests(unittest.TestCase):

    def test_special_pairs(self):
        self.assertEqual(special_pairs(18), [[5, 13], [7, 11]])
        self.assertEqual(special_pairs(2), [])
        self.assertEqual(len(special_pairs(100)), 6)

    def test_tree_constructor(self):
        self.assertTrue(tree_constructor(["(1,2)", "(2,4)", "(5,7)", "(7,2)", "(9,5)"]))
        self.assertFalse(tree_constructor(["(1,2)", "(3,2)", "(2,12)", "(5,2)"]))
        self.assertFalse(tree_constructor([]))
        

if __name__ == "__main__":
    unittest.main()