class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        head = ListNode(0)  # 创建一个哑节点，方便处理结果链表的头节点
        current = head  # 当前节点指针
        carry = 0  # 进位

        while l1 or l2 or carry:
            # 获取当前节点的值，如果链表已经遍历完，则值为0
            x = l1.val if l1 else 0
            y = l2.val if l2 else 0

            # 计算当前位的和以及进位
            total = x + y + carry
            carry = total // 10
            digit = total % 10

            # 创建新节点并连接到结果链表
            current.next = ListNode(digit)
            current = current.next

            # 移动到下一个节点
            if l1:
                l1 = l1.next
            if l2:
                l2 = l2.next

        return head.next  # 返回结果链表的头节点
