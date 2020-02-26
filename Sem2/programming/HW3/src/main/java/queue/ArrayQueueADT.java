package queue;



public class ArrayQueueADT {

    private int tail = 0;
    private int head = 0;

    private int length = 0;

    private Object[] array = new Object[2];

    private static void expandArrayNeeded(ArrayQueueADT queue) {
        if(queue.array[queue.tail] == null) {
            return;
        }
        Object[] temp = new Object[queue.array.length*2];
        System.arraycopy(queue.array, queue.head, temp, queue.array.length + queue.head, queue.array.length - queue.head);
        System.arraycopy(queue.array, 0, temp, 0, queue.tail);
        queue.head = queue.array.length + queue.head;
        queue.array = temp;
    }

    public static void enqueue(ArrayQueueADT queue, Object x) {
        expandArrayNeeded(queue);
        queue.array[queue.tail] = x;
        queue.tail = (queue.tail+1) % queue.array.length;
        queue.length++;
        // printArray();
    }

    public static Object dequeue(ArrayQueueADT queue) {
        if(queue.array[queue.head] == null) {
            return null;
        }
        Object t = queue.array[queue.head];
        queue.array[queue.head] = null;
        queue.head = (queue.head + 1) % queue.array.length;
        queue.length--;
        // printArray(ArrayQueueADT queue);
        return t;
    }

    public static Object element(ArrayQueueADT queue) {
        return queue.array[queue.head];
    }

    public static int size(ArrayQueueADT queue) {
        return queue.length;
    }

    public static boolean isEmpty(ArrayQueueADT queue) {
        return queue.length == 0;
    }
    public static void clear(ArrayQueueADT queue) {
        queue.array = new Object[2];
        queue.length = 0;
        queue.head = 0;
        queue.tail = 0;
    }
}
