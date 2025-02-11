# client.py
import grpc
import message_pb2
import message_pb2_grpc
import logging

def get_user(user_id: int):
    with grpc.insecure_channel('localhost:50051') as channel:
        stub = message_pb2_grpc.UserServiceStub(channel)
        request = message_pb2.UserRequest(user_id=user_id)
        try:
            response = stub.GetUser(request)
            if response.success:
                user = response.user
                print(f"Found user: {user.name} (ID: {user.id})")
                print(f"Email: {user.email}")
                print(f"Roles: {', '.join(user.roles)}")
            else:
                print(f"Error: {response.error_message}")
            return response
        except grpc.RpcError as e:
            logging.error(f"RPC failed: {e}")
            return None

if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO)
    # Test with existing and non-existing user
    get_user(1)
    get_user(2)