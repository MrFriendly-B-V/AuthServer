package nl.thedutchmc.authserver;

public class User {
	private String email, id, token, refreshToken, sessionId;
	
	public User(String email, String id, String token, String refreshToken) {
		this.email = email;
		this.id = id;
		this.token = token;
		this.refreshToken = refreshToken;
	}
	
	public String getEmail() {
		return this.email;
	}
	
	public String getId() {
		return this.id;
	}
	
	public String getToken() {
		return this.token;
	}
	
	public void setToken(String token) {
		this.token = token;
	}
	
	public String getRefreshToken() {
		return this.refreshToken;
	}
	
	public String getSessionId() {
		return this.sessionId;
	}
	
	public void setSessionId(String sessionId) {
		this.sessionId = sessionId;
	}
	
}
