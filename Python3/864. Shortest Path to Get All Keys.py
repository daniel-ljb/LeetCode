class Solution:
    def shortestPathAllKeys(self, grid: List[str]) -> int:
        from collections import deque

        rows = len(grid)
        columns = len(grid[0])

        startRow, startColumn = 0, 0
        numberOfKeys = 0

        for i in range(rows):
            for j in range(columns):
                if grid[i][j] == "@":
                    startRow = i
                    startColumn = j
                elif grid[i][j].islower():
                    numberOfKeys += 1
        
        directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]

        queue = deque([(startRow, startColumn, 0, "")])
        visited = set()

        while queue:
            row, column, distance, collectedKeys = queue.popleft()

            for dx, dy in directions:
                newRow = row + dy
                newColumn = column + dx

                if not(0 <= newRow < rows and 0 <= newColumn < columns):
                    continue
                
                cell = grid[newRow][newColumn]

                if cell == "#" or cell.isupper() and cell.lower() not in collectedKeys:
                    continue

                newKeys = collectedKeys
                if cell.islower() and cell not in collectedKeys:
                    newKeys += cell
                    newKeys = "".join(sorted(newKeys))

                if len(newKeys) == numberOfKeys:
                    return distance + 1
                
                if (newRow, newColumn, newKeys) in visited:
                    continue
                
                visited.add((newRow, newColumn, newKeys))
                queue.append((newRow, newColumn, distance + 1, newKeys))
        
        return -1


