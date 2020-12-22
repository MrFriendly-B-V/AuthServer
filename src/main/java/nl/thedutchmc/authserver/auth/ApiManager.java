package nl.thedutchmc.authserver.auth;

import java.util.ArrayList;
import java.util.List;

public class ApiManager {

	private static List<String> validApiTokens = new ArrayList<>();
	
	public ApiManager(List<String> validApiTokens) {
		ApiManager.validApiTokens = validApiTokens;
	}
	
	public static boolean isApiTokenValid(String apiToken) {
		return validApiTokens.contains(apiToken);
	}
}
