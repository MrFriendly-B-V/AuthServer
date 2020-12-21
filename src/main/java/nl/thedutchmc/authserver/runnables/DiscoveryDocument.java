package nl.thedutchmc.authserver.runnables;

import java.io.IOException;
import java.net.MalformedURLException;

import org.apache.commons.lang3.exception.ExceptionUtils;
import org.json.JSONObject;

import nl.thedutchmc.authserver.App;
import nl.thedutchmc.authserver.Http;
import nl.thedutchmc.authserver.Http.RequestMethod;
import nl.thedutchmc.authserver.Http.ResponseObject;

public class DiscoveryDocument implements Runnable {

	@Override
	public void run() {
		
		App.logInfo("Getting Authentication Endpoint from Google...");
		
		Http http = new Http();
		ResponseObject responseObject = null;
		try {
			responseObject = http.makeRequest(RequestMethod.GET, "https://accounts.google.com/.well-known/openid-configuration", null, null, null);
		} catch (MalformedURLException e) {
			App.logError("Unable to get authentication endpoint. Caused by MalformedURLException. Exiting");
			App.logDebug(ExceptionUtils.getStackTrace(e));
			System.exit(1);
		} catch (IOException e) {
			App.logError("Unable to get authentication endpoint. Caused by IOException. Exiting");
			App.logDebug(ExceptionUtils.getStackTrace(e));
			System.exit(1);
		}
		
		JSONObject responseJson = new JSONObject(responseObject.getMessage());
		App.authEndpoint = responseJson.getString("token_endpoint");
		
		App.logInfo("Authentication endpoint received.");
	}
}
