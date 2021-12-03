#### 文档说明
这是一个Rust学习练习的文档，其中包含了Rust的基础知识，编程测试用例等，方便初学者能快速上手，打好坚实的基本功，
工欲善其事，必先利其器。
精力是有限的，用对方向很重要！

#### 1. 

```

# failed to authenticate when downloading repository: 
# ssh://git@github.com/rust-lang/crates.io-index
# 报错是时处理方法

eval `ssh-agent -s` && ssh-add

```

#### rust exercises

```
echo "# rust-exercises" >> README.md
git init
git add README.md
git commit -m "first commit"
git branch -M main
git remote add origin git@github.com:xuguangtech/rust-exercises.git
git push -u origin main

```

```
git remote add origin git@github.com:xuguangtech/rust-exercises.git
git branch -M main
git push -u origin main

```
