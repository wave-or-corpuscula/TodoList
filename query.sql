WITH RECURSIVE task_tree AS (
        SELECT * FROM Task WHERE parent_id IS NULL
        UNION ALL
        SELECT t.* FROM Task t
        JOIN task_tree tt ON t.parent_id = tt.id
    )
    SELECT * FROM task_tree 
    WHERE completed = ?1 
    ORDER BY creation_date