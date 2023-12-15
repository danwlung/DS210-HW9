# DS210-HW9

This is my DS210 HW9 Fall 2023,

Full Credit 40/40

1. (40 points) Input: Your program should read a set of points in R with labels from data.txt
(you will create file data.txt). Each line of data.txt describes one point. More specifically,
the i-th line consists of two numbers xi and zi, where xi
is an integer s.t. |xi
| ≤ 100,000,000
and zi ∈ {0, 1}. xi
is the coordinate of the point and zi
is its label. Make sure your file
contains at least 100 points randomly distributed in the allowed space with labels
assigned to each point in a random fashion as well.
Your task: Write a program that reads the data and determines a decision tree with at
most two leaves that performs best at predicting zi based on xi on this set of points. The
program should output the decision tree and its accuracy on the input data set (no need
to split on a training and testing set though you are welcome to do so if you want).
Report: Explain why your solution works and what complexity it has as a function of the
number of points (denote this quantity by n).
Sample input and output: For the following data.txt:
-15 0
15 1
-5 1
5 0
the following output is a correct solution:
if x >= 10
output 1
else
output 0
accuracy: 0.75
Hint: You do not need to implement the whole gini logic here. Your input is
1-dimensional so you need to find a split point that minimizes error.
2. (Optional, no credit) Report: How much time did you spend on this homework? The
answer will have no impact on the credit you receive, but it may help us adjust the
difficulty of future homework assignments.
3. (Optional, no credit)
Report and/or code: How can you solve Question 1 when more than two leaves are
allowed? What is the complexity of your solution as a function of the numbers of leaves
and points?
Hint: Once you have decided on a split there is no backtracking in decision tree logic. All
you can do is decide how to split the two sides further.
