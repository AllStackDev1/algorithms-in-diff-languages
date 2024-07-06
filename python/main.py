from special_pairs import special_pairs
from tree_constructor import tree_constructor


def main(): 
    # print("Special Pairs - PY %s" % special_pairs(18))
    print("Tree Constructor - TS %s" % tree_constructor(["(1,2)", "(2,4)", "(5,7)", "(7,2)", "(9,5)"]))
    print("Tree Constructor - TS %s" % tree_constructor(["(1,2)", "(3,2)", "(2,12)", "(5,2)"]))
    pass

if __name__ == '__main__':
    main()