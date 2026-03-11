#!/bin/bash

# 前30个练习文件的路径
EXERCISES=(
    "intro/intro1.rs"
    "intro/intro2.rs"
    "variables/variables1.rs"
    "variables/variables2.rs"
    "variables/variables3.rs"
    "variables/variables4.rs"
    "variables/variables5.rs"
    "variables/variables6.rs"
    "functions/functions1.rs"
    "functions/functions2.rs"
    "functions/functions3.rs"
    "functions/functions4.rs"
    "functions/functions5.rs"
    "if/if1.rs"
    "if/if2.rs"
    "if/if3.rs"
    "primitive_types/primitive_types1.rs"
    "primitive_types/primitive_types2.rs"
    "primitive_types/primitive_types3.rs"
    "primitive_types/primitive_types4.rs"
    "primitive_types/primitive_types5.rs"
    "primitive_types/primitive_types6.rs"
    "vecs/vecs1.rs"
    "vecs/vecs2.rs"
    "structs/structs1.rs"
    "structs/structs2.rs"
    "structs/structs3.rs"
    "strings/strings1.rs"
    "strings/strings2.rs"
    "strings/strings3.rs"
)

# 遍历每个练习文件
for exercise in "${EXERCISES[@]}"; do
    file_path="/home/mikasa/2026s-rustling-2944916045-star/exercises/$exercise"
    
    # 检查文件是否存在
    if [ -f "$file_path" ]; then
        # 检查文件是否已经包含 // I AM NOT DONE 注释
        if ! grep -q "// I AM NOT DONE" "$file_path"; then
            # 在文件的适当位置添加注释
            # 找到第一个空行，在其前添加注释
            sed -i '/^$/i\// I AM NOT DONE' "$file_path"
            echo "Added // I AM NOT DONE to $exercise"
        else
            echo "$exercise already has // I AM NOT DONE"
        fi
    else
        echo "File $exercise not found"
    fi
done

echo "Reset completed!"
