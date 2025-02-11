# server.py
from concurrent import futures
import grpc
import message_pb2
import message_pb2_grpc
import logging

class UserServicer(message_pb2_grpc.UserServiceServicer):
    def GetUser(self, request, context):
        logging.info(f"Received request for user_id: {request.user_id}")
        
        # Simulate database lookup
        if request.user_id == 1:
            user = message_pb2.User(
                id=1,
                name="John Doe",
                email="john@example.com",
                roles=["user", "admin"]
            )
            return message_pb2.UserResponse(user=user, success=True)
        else:
            return message_pb2.UserResponse(
                success=False,
                error_message=f"User {request.user_id} not found"
            )

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    message_pb2_grpc.add_UserServiceServicer_to_server(UserServicer(), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    logging.info("Server started on port 50051")
    server.wait_for_termination()

if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO)
    serve()