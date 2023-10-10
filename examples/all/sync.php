<?php
    foreach (['A', 'B', 'C', 'D'] as $user) {
        var_dump(file_get_contents("http://localhost:3001/$user"));
    }
?>
