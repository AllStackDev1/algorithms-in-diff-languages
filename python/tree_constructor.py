def tree_constructor(strArr) -> bool: 
    parents = {}
    children = {}

    for pair in strArr:
        child, parent = pair.replace("(", "").replace(")", "").split(",")

        if parent not in parents:
            parents[parent] = []
        
        parents[parent].append(child)

        # Check if the parent has more than 2 children (invalid binary tree)
        if len(parents[parent]) > 2:
          return False
        
        # Check if the child already has a parent (invalid binary tree)
        if child in children:
          return False
        
        children[child] = parent

    rootCount = len(list(filter(lambda parent: all(child not in parents for child in parents[parent]), parents)))
    # for parent in parents:
    #    if parent not in children:
    #       rootCount += 1 

    # There should be exactly one root for a valid binary tree
    return rootCount == 1
