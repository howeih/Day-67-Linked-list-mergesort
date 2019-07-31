Day 67: Linked-list mergesort
<br>
Merge sort is often preferred for sorting a linked list. The slow random-access performance of a linked list makes some other algorithms (such as quicksort) perform poorly, and others (such as heapsort) completely impossible.

![Alt text](sort_linked_list.png?raw=true "sort_linked_list")

<br>
Let head be the first node of the linked list to be sorted and headRef be the pointer to head. Note that we need a reference to head in MergeSort() as the below implementation changes next links to sort the linked lists (not data at the nodes), so head node has to be changed if the data at the original head is not the smallest value in the linked list.
<br>
```$xslt
MergeSort(headRef)
1) If the head is NULL or there is only one element in the Linked List 
    then return.
2) Else divide the linked list into two halves.  
      FrontBackSplit(head, &a, &b); /* a and b are two halves */
3) Sort the two halves a and b.
      MergeSort(a);
      MergeSort(b);
4) Merge the sorted a and b (using SortedMerge() discussed here) 
   and update the head pointer using headRef.
     *headRef = SortedMerge(a, b);
```

run:
```
fn main() {
    let mut linked_list = LinkedList::new();
    linked_list.push(15);
    linked_list.push(10);
    linked_list.push(5);
    linked_list.push(20);
    linked_list.push(3);
    linked_list.push(2);
    println!("before sort:");
    linked_list.print_list();
    let linked_list_len = linked_list.len;
    merge_sort(&mut linked_list, 0, linked_list_len / 2, linked_list_len);
    println!("after sort:");
    linked_list.print_list();
}

```
result:
```
before sort:
index:     0    1    2    3    4    5
value:    15   10    5   20    3    2
after sort:
index:     0    1    2    3    4    5
value:     2    3    5   10   15   20
```