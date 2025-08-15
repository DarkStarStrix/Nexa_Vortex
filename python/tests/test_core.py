import time
import threading
from uuid import UUID
from nexa_vortex.message import Message
from nexa_vortex import VortexWorkQueue, CpuDispatcher, ControlPlane

def test_message_serialization():
    """Tests that a Message can be serialized to and from a dictionary."""
    msg = Message(payload={"data": "test"})
    msg_dict = msg.to_dict()

    assert isinstance(msg_dict["id"], str)
    assert msg_dict["payload"] == {"data": "test"}

    rehydrated_msg = Message.from_dict(msg_dict)
    assert isinstance(rehydrated_msg.id, UUID)
    assert rehydrated_msg.id == msg.id
    assert rehydrated_msg.payload == msg.payload

def test_work_queue_py():
    """Tests the basic functionality of the VortexWorkQueue from Python."""
    q = VortexWorkQueue(10)
    q.push(100)
    q.push("hello")
    assert q.pop() == 100
    assert q.pop() == "hello"
    assert q.pop() is None

def test_cpu_dispatcher_py():
    """Tests dispatching a simple lambda function to the Rust thread pool."""
    dispatcher = CpuDispatcher(num_threads=2)

    # Use a thread-safe list to check for side effects
    results = []
    lock = threading.Lock()

    def task():
        with lock:
            results.append(1)

    dispatcher.dispatch(task)

    # Give the dispatcher time to execute the task
    time.sleep(0.1)

    with lock:
        assert len(results) == 1
        assert results[0] == 1

def test_control_plane_integration():
    """Tests the control plane's ability to dispatch a command."""
    dispatcher = CpuDispatcher(num_threads=1)
    control_plane = ControlPlane(dispatcher=dispatcher)

    results = []
    lock = threading.Lock()
    def command_task():
        with lock:
            results.append("executed")

    control_plane.dispatcher.dispatch(command_task)

    # Give the task time to run
    time.sleep(0.1)

    with lock:
        assert results == ["executed"]
