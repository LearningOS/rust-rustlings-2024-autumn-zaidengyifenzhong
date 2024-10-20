/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]  
pub struct Queue<T> {  
    elements: Vec<T>,  
}  
  
impl<T> Queue<T> {  
    pub fn new() -> Queue<T> {  
        Queue {  
            elements: Vec::new(),  
        }  
    }  
  
    pub fn enqueue(&mut self, value: T) {  
        self.elements.push(value);  
    }  
  
    pub fn dequeue(&mut self) -> Result<T, &str> {  
        if !self.elements.is_empty() {  
            Ok(self.elements.remove(0))  
        } else {  
            Err("Queue is empty")  
        }  
    }  
  
    pub fn peek(&self) -> Result<&T, &str> {  
        match self.elements.first() {  
            Some(value) => Ok(value),  
            None => Err("Queue is empty"),  
        }  
    }  
  
    pub fn size(&self) -> usize {  
        self.elements.len()  
    }  
  
    pub fn is_empty(&self) -> bool {  
        self.elements.is_empty()  
    }  
}  
  
impl<T> Default for Queue<T> {  
    fn default() -> Queue<T> {  
        Queue::new()  
    }  
}  
  
pub struct MyStack<T> {  
    q1: Queue<T>,  
    q2: Queue<T>,  
}  
  
impl<T> MyStack<T> {  
    pub fn new() -> Self {  
        Self {  
            q1: Queue::new(),  
            q2: Queue::new(),  
        }  
    }  
  
    pub fn push(&mut self, elem: T) {  
        // 总是将元素推送到q1  
        self.q1.enqueue(elem);  
    }  
  
    pub fn pop(&mut self) -> Result<T, &str> {  
        // 如果q1为空，则无法弹出元素  
        if self.q1.is_empty() {  
            return Err("Stack is empty");  
        }  
  
        // 将q1中的元素（除了最后一个）逐个移动到q2  
        while self.q1.size() > 1 {  
            self.q2.enqueue(self.q1.dequeue().unwrap());  
        }  
  
        // q1中剩下的最后一个元素就是栈顶元素，将其弹出并返回  
        let top_elem = self.q1.dequeue().unwrap();  
  
        // 交换q1和q2的角色，以便下次push时可以继续使用q1  
        std::mem::swap(&mut self.q1, &mut self.q2);  
  
        // 清空刚刚被用作临时存储的队列（现在是q2）  
        while let Ok(value) = self.q2.dequeue() {  
            self.q1.enqueue(value); // 这些元素会留在q1中，直到下次被弹出  
        }  
  
        Ok(top_elem)  
    }  
  
    pub fn is_empty(&self) -> bool {  
        // 栈为空当且仅当q1为空  
        self.q1.is_empty()  
    }  
}  
  
#[cfg(test)]  
mod tests {  
    use super::*;  
  
    #[test]  
    fn test_stack() {  
        let mut s = MyStack::<i32>::new();  
        assert_eq!(s.pop(), Err("Stack is empty"));  
        s.push(1);  
        s.push(2);  
        s.push(3);  
        assert_eq!(s.pop(), Ok(3));  
        assert_eq!(s.pop(), Ok(2));  
        s.push(4);  
        s.push(5);  
        assert_eq!(s.is_empty(), false);  
        assert_eq!(s.pop(), Ok(5));  
        assert_eq!(s.pop(), Ok(4));  
        assert_eq!(s.pop(), Ok(1));  
        assert_eq!(s.pop(), Err("Stack is empty"));  
        assert_eq!(s.is_empty(), true);  
    }  
}