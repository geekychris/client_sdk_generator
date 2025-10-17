package com.example.client;

import dev.failsafe.Failsafe;
import dev.failsafe.RetryPolicy;
import io.micrometer.core.instrument.Counter;
import io.micrometer.core.instrument.MeterRegistry;
import io.micrometer.core.instrument.Timer;
import io.micrometer.prometheus.PrometheusConfig;
import io.micrometer.prometheus.PrometheusMeterRegistry;
import java.time.Duration;

// gRPC imports
import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import io.grpc.StatusRuntimeException;
import io.grpc.stub.StreamObserver;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.Iterator;

// Generated proto imports (would be generated from .proto files)
// import .UserServiceGrpc;
// import .*;

/**
 * Generated from UserService gRPC proto files
 * 
 * Generated gRPC client for UserService gRPC API v1.0.0
 */
public class UserserviceGrpcApiClient {
    
    private final ManagedChannel channel;
    private final ExecutorService executor;
    // In real implementation, these would be generated from proto:
    // private final UserServiceGrpc.UserServiceBlockingStub blockingStub;
    // private final UserServiceGrpc.UserServiceStub asyncStub;
    
    private final TelemetryHandler telemetryHandler;
    
    
    public UserserviceGrpcApiClient(String host, int port) {
        this.channel = ManagedChannelBuilder.forAddress(host, port)
            .usePlaintext() // Use .useTransportSecurity() for TLS
            .build();
        this.executor = Executors.newFixedThreadPool(4);
        
        // In real implementation:
        // this.blockingStub = UserServiceGrpc.newBlockingStub(channel);
        // this.asyncStub = UserServiceGrpc.newStub(channel);
        
        this.telemetryHandler = new TelemetryHandler();
        
    }
    
    // Generated gRPC service methods
    
    /**
     * Register a new user account
     * gRPC method: RegisterUser
     */
    public String registeruser(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.registeruser(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method RegisterUser not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("RegisterUser", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call RegisterUser failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("RegisterUser", "grpc", duration, true);
        }
    }
    
    /**
     * Register a new user account (Async)
     * gRPC method: RegisterUser
     */
    public CompletableFuture<String> registeruserAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return registeruser(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Authenticate a user and create session
     * gRPC method: LoginUser
     */
    public String loginuser(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.loginuser(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method LoginUser not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("LoginUser", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call LoginUser failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("LoginUser", "grpc", duration, true);
        }
    }
    
    /**
     * Authenticate a user and create session (Async)
     * gRPC method: LoginUser
     */
    public CompletableFuture<String> loginuserAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return loginuser(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Refresh authentication token
     * gRPC method: RefreshToken
     */
    public String refreshtoken(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.refreshtoken(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method RefreshToken not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("RefreshToken", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call RefreshToken failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("RefreshToken", "grpc", duration, true);
        }
    }
    
    /**
     * Refresh authentication token (Async)
     * gRPC method: RefreshToken
     */
    public CompletableFuture<String> refreshtokenAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return refreshtoken(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Logout user and invalidate session
     * gRPC method: LogoutUser
     */
    public String logoutuser(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.logoutuser(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method LogoutUser not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("LogoutUser", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call LogoutUser failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("LogoutUser", "grpc", duration, true);
        }
    }
    
    /**
     * Logout user and invalidate session (Async)
     * gRPC method: LogoutUser
     */
    public CompletableFuture<String> logoutuserAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return logoutuser(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Get user profile by ID
     * gRPC method: GetUser
     */
    public String getuser(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.getuser(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method GetUser not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("GetUser", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call GetUser failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("GetUser", "grpc", duration, true);
        }
    }
    
    /**
     * Get user profile by ID (Async)
     * gRPC method: GetUser
     */
    public CompletableFuture<String> getuserAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return getuser(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Get current authenticated user profile
     * gRPC method: GetCurrentUser
     */
    public String getcurrentuser(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.getcurrentuser(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method GetCurrentUser not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("GetCurrentUser", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call GetCurrentUser failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("GetCurrentUser", "grpc", duration, true);
        }
    }
    
    /**
     * Get current authenticated user profile (Async)
     * gRPC method: GetCurrentUser
     */
    public CompletableFuture<String> getcurrentuserAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return getcurrentuser(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Update user profile information
     * gRPC method: UpdateUser
     */
    public String updateuser(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.updateuser(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method UpdateUser not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("UpdateUser", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call UpdateUser failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("UpdateUser", "grpc", duration, true);
        }
    }
    
    /**
     * Update user profile information (Async)
     * gRPC method: UpdateUser
     */
    public CompletableFuture<String> updateuserAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return updateuser(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Delete user account
     * gRPC method: DeleteUser
     */
    public String deleteuser(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.deleteuser(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method DeleteUser not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("DeleteUser", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call DeleteUser failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("DeleteUser", "grpc", duration, true);
        }
    }
    
    /**
     * Delete user account (Async)
     * gRPC method: DeleteUser
     */
    public CompletableFuture<String> deleteuserAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return deleteuser(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * List users with pagination and filtering
     * gRPC method: ListUsers
     */
    public String listusers(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.listusers(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method ListUsers not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("ListUsers", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call ListUsers failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("ListUsers", "grpc", duration, true);
        }
    }
    
    /**
     * List users with pagination and filtering (Async)
     * gRPC method: ListUsers
     */
    public CompletableFuture<String> listusersAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return listusers(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Change user password
     * gRPC method: ChangePassword
     */
    public String changepassword(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.changepassword(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method ChangePassword not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("ChangePassword", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call ChangePassword failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("ChangePassword", "grpc", duration, true);
        }
    }
    
    /**
     * Change user password (Async)
     * gRPC method: ChangePassword
     */
    public CompletableFuture<String> changepasswordAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return changepassword(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Reset user password
     * gRPC method: ResetPassword
     */
    public String resetpassword(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.resetpassword(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method ResetPassword not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("ResetPassword", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call ResetPassword failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("ResetPassword", "grpc", duration, true);
        }
    }
    
    /**
     * Reset user password (Async)
     * gRPC method: ResetPassword
     */
    public CompletableFuture<String> resetpasswordAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return resetpassword(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Send email verification
     * gRPC method: SendVerificationEmail
     */
    public String sendverificationemail(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.sendverificationemail(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method SendVerificationEmail not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("SendVerificationEmail", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call SendVerificationEmail failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("SendVerificationEmail", "grpc", duration, true);
        }
    }
    
    /**
     * Send email verification (Async)
     * gRPC method: SendVerificationEmail
     */
    public CompletableFuture<String> sendverificationemailAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return sendverificationemail(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Verify email address
     * gRPC method: VerifyEmail
     */
    public String verifyemail(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.verifyemail(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method VerifyEmail not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("VerifyEmail", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call VerifyEmail failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("VerifyEmail", "grpc", duration, true);
        }
    }
    
    /**
     * Verify email address (Async)
     * gRPC method: VerifyEmail
     */
    public CompletableFuture<String> verifyemailAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return verifyemail(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * List user active sessions
     * gRPC method: ListUserSessions
     */
    public String listusersessions(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.listusersessions(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method ListUserSessions not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("ListUserSessions", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call ListUserSessions failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("ListUserSessions", "grpc", duration, true);
        }
    }
    
    /**
     * List user active sessions (Async)
     * gRPC method: ListUserSessions
     */
    public CompletableFuture<String> listusersessionsAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return listusersessions(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Revoke a user session
     * gRPC method: RevokeSession
     */
    public String revokesession(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.revokesession(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method RevokeSession not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("RevokeSession", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call RevokeSession failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("RevokeSession", "grpc", duration, true);
        }
    }
    
    /**
     * Revoke a user session (Async)
     * gRPC method: RevokeSession
     */
    public CompletableFuture<String> revokesessionAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return revokesession(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Get user preferences
     * gRPC method: GetUserPreferences
     */
    public String getuserpreferences(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.getuserpreferences(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method GetUserPreferences not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("GetUserPreferences", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call GetUserPreferences failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("GetUserPreferences", "grpc", duration, true);
        }
    }
    
    /**
     * Get user preferences (Async)
     * gRPC method: GetUserPreferences
     */
    public CompletableFuture<String> getuserpreferencesAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return getuserpreferences(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Update user preferences
     * gRPC method: UpdateUserPreferences
     */
    public String updateuserpreferences(String request) throws StatusRuntimeException {
        long startTime = System.currentTimeMillis();
        
        try {
            // Generated gRPC call would be:
            // return blockingStub.updateuserpreferences(request);
            
            // For now, throw unsupported operation with method info
            throw new UnsupportedOperationException(
                "gRPC method UpdateUserPreferences not implemented. " +
                "This client template shows the structure. " +
                "Generate actual gRPC stubs from .proto files using protoc with grpc-java plugin."
            );
        } catch (StatusRuntimeException e) {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("UpdateUserPreferences", "grpc", duration, false);
            throw new UserserviceGrpcApiException("gRPC call UpdateUserPreferences failed: " + e.getStatus(), e);
        } finally {
            long duration = System.currentTimeMillis() - startTime;
            telemetryHandler.recordRequest("UpdateUserPreferences", "grpc", duration, true);
        }
    }
    
    /**
     * Update user preferences (Async)
     * gRPC method: UpdateUserPreferences
     */
    public CompletableFuture<String> updateuserpreferencesAsync(String request) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return updateuserpreferences(request);
            } catch (StatusRuntimeException e) {
                throw new RuntimeException(e);
            }
        }, executor);
    }
    
    /**
     * Shutdown the gRPC client and release resources
     */
    public void shutdown() throws InterruptedException {
        if (channel != null && !channel.isShutdown()) {
            channel.shutdown().awaitTermination(5, TimeUnit.SECONDS);
        }
        if (executor != null && !executor.isShutdown()) {
            executor.shutdown();
            executor.awaitTermination(5, TimeUnit.SECONDS);
        }
    }
    
    /**
     * Force shutdown the gRPC client immediately
     */
    public void shutdownNow() {
        if (channel != null && !channel.isShutdown()) {
            channel.shutdownNow();
        }
        if (executor != null && !executor.isShutdown()) {
            executor.shutdownNow();
        }
    }
    
    /**
     * Check if the channel is terminated
     */
    public boolean isTerminated() {
        return channel == null || channel.isTerminated();
    }
    
    public static class UserserviceGrpcApiException extends RuntimeException {
        public UserserviceGrpcApiException(String message) {
            super(message);
        }
        
        public UserserviceGrpcApiException(String message, Throwable cause) {
            super(message, cause);
        }
    }
}
