package nl.thedutchmc.authserver.runnables;

import java.io.IOException;
import java.net.MalformedURLException;
import java.util.HashMap;
import java.util.concurrent.ScheduledThreadPoolExecutor;
import java.util.concurrent.TimeUnit;

import org.json.JSONObject;

import nl.thedutchmc.authserver.App;
import nl.thedutchmc.authserver.Config;
import nl.thedutchmc.authserver.Http;
import nl.thedutchmc.authserver.User;
import nl.thedutchmc.authserver.Http.RequestMethod;
import nl.thedutchmc.authserver.Http.ResponseObject;

public class RefreshToken implements Runnable {

	private String id, refreshToken;
	
	public RefreshToken(String id, String refreshToken) {
		this.id = id;
		this.refreshToken = refreshToken;
	}
	
	@Override
	public void run() {
		
		App.logInfo("Refreshing token for" + id);
		
		if(refreshToken.equals("")) {
			App.logError("Cannot refresh token for " + id + ". There is no refresh_token!");
		}
		
		//Set all the required URL parameters
		HashMap<String, String> params = new HashMap<>();
		params.put("client_id", Config.clientId);
		params.put("client_secret", Config.clientSecret);
		params.put("grant_type", "refresh_token");
		params.put("refresh_token", refreshToken);
		
		//Make the request to refresh the token
		ResponseObject responseObject = null;
		try {
			responseObject = new Http().makeRequest(RequestMethod.POST, "https://oauth2.googleapis.com/token", params, null, null);
		} catch (MalformedURLException e) {
			e.printStackTrace();
		} catch (IOException e) {
			e.printStackTrace();
		}
		
		if(responseObject.getResponseCode() != 200) {
			App.userMap.remove(id);
			
			new Config().writeStorage();
			
			
			return;
		}
		
		//Parse the response into a JSONObject and get the token and expiresIn		
		JSONObject responseJson = new JSONObject(responseObject.getMessage());
		String token = responseJson.getString("access_token");
		long expiresIn = responseJson.getLong("expires_in");
		
		//Get the existing User object, update it, and put it back in the map
		User user = App.userMap.get(id);
		user.setToken(token);
		App.userMap.put(id, user);
		
		//Update the storage file
		new Config().writeStorage();
		
		//Schedule the next run to be whatever expiresIn is, minus 60 seconds
		Thread refreshTokenThread = new Thread(new RefreshToken(id, refreshToken));		
		final ScheduledThreadPoolExecutor executor = new ScheduledThreadPoolExecutor(3);
		executor.schedule(() -> refreshTokenThread.start(), expiresIn - 60L, TimeUnit.SECONDS);
	}
}
