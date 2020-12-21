package nl.thedutchmc.authserver;

import java.io.IOException;
import java.net.MalformedURLException;
import java.util.Base64;
import java.util.HashMap;
import java.util.concurrent.ScheduledThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.regex.Pattern;

import org.json.JSONObject;

import nl.thedutchmc.authserver.Http.MediaFormat;
import nl.thedutchmc.authserver.Http.RequestMethod;
import nl.thedutchmc.authserver.Http.ResponseObject;
import nl.thedutchmc.authserver.runnables.RefreshToken;
import nl.thedutchmc.authserver.session.SessionManager;

public class TokenManager {

	public void getToken(String code, String sessionId) {
		//Make the request
		ResponseObject responseObject = null;
		try {
			HashMap<String, String> requestBody = new HashMap<>();
			requestBody.put("code", code);
			requestBody.put("client_id", Config.clientId);
			requestBody.put("client_secret", Config.clientSecret);
			requestBody.put("redirect_uri", Config.SCHEME + Config.host + Config.REDIRECT_URI_RESPONSE);
			requestBody.put("grant_type", "authorization_code");			
			
			responseObject = new Http().makeRequest(RequestMethod.POST, App.authEndpoint, null, MediaFormat.X_WWW_FORM_URLENCODED, Http.hashMapToString(requestBody));
		} catch (MalformedURLException e) {
			e.printStackTrace();
		} catch (IOException e) {
			e.printStackTrace();
		}
		
		//JSON Response from the server
		JSONObject responseJson = new JSONObject(responseObject.getMessage());
		
		//Get the values from the response JSON
		String token = responseJson.getString("access_token");
		long expiresIn = responseJson.getLong("expires_in");
		String jwt = responseJson.getString("id_token");
		
		//TODO we should probably verify the JWT:
		// - Verify that the `iss` is `https://accounts.google.com` or `accounts.google.com`
		// - Verify the signature, public cert can be found at whatever `jwks_uri` is
		// - Verify that the expiry time is not yet passed, expiry time is whatever `exp` is
		//See: https://developers.google.com/identity/protocols/oauth2/openid-connect#validatinganidtoken
		
		//Decode the JWT we got from the server
		//JWT Format {signature specification}.{content}.{signature}
		//We only care for the content, so split that out
		//It is in base64 though, so we have to decode it before we can use it.
		String jwtDataEncoded = jwt.split(Pattern.quote("."))[1];
		String jwtDataDecoded = new String(Base64.getDecoder().decode(jwtDataEncoded));
		
		//Get the email and user id from the decoded JWT
		JSONObject jwtDecoded = new JSONObject(jwtDataDecoded);
		String email = jwtDecoded.getString("email");
		String id = jwtDecoded.getString("sub");
		
		String refreshToken = "";
		if(responseJson.has("refresh_token")) {
			refreshToken = responseJson.getString("refresh_token");
		}
		
		User user = App.userMap.get(id);
		
		if(user == null) {
			//Create a new User object
			user = new User(email, id, token, refreshToken);
		} else {
			user.setToken(token);
		}
		
		//Add the new user to the hashMap
		App.userMap.put(id, user);
		
		new Config().writeStorage();
		
		//We also want to add the newly acquired data into the database
		new SessionManager().createSession(id, refreshToken, sessionId);
		
		if(!refreshToken.equals("")) {
			//Start a new Thread to be ran when the token expires
			Thread refreshTokenThread = new Thread(new RefreshToken(id, refreshToken));		
			final ScheduledThreadPoolExecutor executor = new ScheduledThreadPoolExecutor(3);
			executor.schedule(() -> refreshTokenThread.start(), expiresIn - 60L, TimeUnit.SECONDS);
		} else {
			App.logInfo("Google did not give us a refresh_token for " + email + ". They will no longer be authenticated in " + expiresIn + " seconds!");
		}
	}
}
