SELECT project.name AS name, sum(log_entry.hours) AS hours 
    FROM project 
    JOIN log_entry ON project_id = project.id 
    WHERE log_entry.user_id = $1
    GROUP BY project.name;