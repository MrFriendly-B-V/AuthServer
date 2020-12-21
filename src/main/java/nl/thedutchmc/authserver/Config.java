package nl.thedutchmc.authserver;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.net.URISyntaxException;
import java.util.List;

import org.apache.commons.lang3.SystemUtils;
import org.apache.commons.lang3.exception.ExceptionUtils;
import org.json.JSONArray;
import org.json.JSONObject;

import com.google.common.base.Charsets;
import com.google.common.io.Files;

import nl.thedutchmc.authserver.runnables.RefreshToken;

public class Config {

	public static String clientSecret = "";
	public static String clientId = "";
	public static String host = "";
	
	public static String mysqlHost;
	public static String mysqlDb;
	public static String mysqlUser;
	public static String mysqlPassword;
	
	public static final String RESPONSE_TYPE = "code";
	public static final String SCHEME = "https://";
	public static final String REDIRECT_URI_RESPONSE = "/oauth/grant";
	public static final String GOOGLE_AUTH_URL = "https://accounts.google.com/o/oauth2/v2/auth";
	
	public static final String[] SCOPES = {
			"https://mail.google.com/",
			"openid profile email"
	};
	
	private static String configDirPath = "";
	
	public Config() {
		//Determine the configuration directory
		// For linux: /etc/espoGmailSync
		// For Windows: C:\Program Files\Espo Gmail Sync
		// For other OS's: Directory where the JAR file of this program is located
		if(SystemUtils.IS_OS_LINUX) {
			configDirPath = "/etc/espoGmailSync";
		} else if(SystemUtils.IS_OS_WINDOWS) {
			configDirPath = "C:\\Program Files\\Espo Gmail Sync";
		} else {
			try {
				final File jarPath = new File(Config.class.getProtectionDomain().getCodeSource().getLocation().toURI().getPath());
				final File folderPath = new File(jarPath.getParentFile().getPath());
				configDirPath = folderPath.getAbsolutePath();
			} catch(URISyntaxException e) {
				App.logError("Unable to determine config path due to an URISyntaxException. Exiting!");
				App.logDebug(e.getStackTrace());
				
				System.exit(1);
			}
		}		
	}
	
	public void readConfig() {
		App.logInfo("Reading config file...");

		File configFile = new File(configDirPath, "config.json");
		
		//Check if the config file exists,
		//if not create it and write the default config
		if(!configFile.exists()) {
			try {
				File configDir = new File(configDirPath);
				configDir.mkdirs();
				
				configFile.createNewFile();
				
				BufferedWriter bw = new BufferedWriter(new FileWriter(configFile));
				
				JSONObject defaultConfig = new JSONObject();
				defaultConfig.put("clientSecret", "");
				defaultConfig.put("clientId", "");
				defaultConfig.put("baseDomain", "");
				defaultConfig.put("mysqlHost", "");
				defaultConfig.put("mysqlDb", "");
				defaultConfig.put("mysqlUser", "");
				defaultConfig.put("mysqlPassword", "");
						
				bw.write(defaultConfig.toString());
				bw.flush();
				bw.close();
			} catch (IOException e) {
				App.logError("Unable to create config.json due to an IOException. Exiting");
				App.logDebug(ExceptionUtils.getStackTrace(e));
				
				System.exit(1);
			}
		}
		
		//We can now read the config
		List<String> fileContentList = null;
		try {
			fileContentList = Files.readLines(configFile, Charsets.UTF_8);
		} catch (IOException e) {
			App.logError("Unable to read config.json due to an IOException. Exiting");
			App.logDebug(ExceptionUtils.getStackTrace(e));
			
			System.exit(1);
		}
		
		//Parse the config file
		JSONObject configJson = new JSONObject(String.join("", fileContentList));
		
		//Read the config options we want
		clientSecret = configJson.getString("clientSecret");
		clientId = configJson.getString("clientId");
		host = configJson.getString("baseDomain");
		mysqlHost = configJson.getString("mysqlHost");
		mysqlDb = configJson.getString("mysqlDb");
		mysqlUser = configJson.getString("mysqlUser");
		mysqlPassword = configJson.getString("mysqlPassword");
		
		App.logInfo("Completed reading configuration file.");
	}
	
	public void readStorage() {
		App.logInfo("Reading storage file...");
		
		File storageFile = new File(configDirPath, "storage.json");
		
		//Check if the storage file exists,
		//if not create it and write the default storage contents
		if(!storageFile.exists()) {
			try {
				File configDir = new File(configDirPath);
				configDir.mkdirs();
				
				storageFile.createNewFile();
				
				BufferedWriter bw = new BufferedWriter(new FileWriter(storageFile));
				
				String[] defaultConfigJson = {
					"{",
					"    \"storage\": []",
					"}"
				};
						
				bw.write(String.join("\n", defaultConfigJson));
				bw.flush();
				bw.close();
			} catch (IOException e) {
				App.logError("Unable to create storage.json due to an IOException. Exiting");
				App.logDebug(ExceptionUtils.getStackTrace(e));
				
				System.exit(1);
			}
		}
		
		//We can now read the storage file
		List<String> fileContentList = null;
		try {
			fileContentList = Files.readLines(storageFile, Charsets.UTF_8);
		} catch (IOException e) {
			App.logError("Unable to read storage.json due to an IOException. Exiting");
			App.logDebug(ExceptionUtils.getStackTrace(e));
			
			System.exit(1);
		}
		
		//Parse the config file
		JSONObject storageJson = new JSONObject(String.join("", fileContentList));
		JSONArray storageContents = storageJson.getJSONArray("storage");
		
		//Iterate over the storage contents
		for(Object o : storageContents) {
			JSONObject storageObject = (JSONObject) o;	
			
			//Get the values
			String id = storageObject.getString("id");
			String email = storageObject.getString("email");
			String refreshToken = storageObject.getString("refreshToken");
			
			//Construct a new User object,
			//We dont have a token, so leave that empty
			User user = new User(email, id, "", refreshToken);
			App.userMap.put(id, user);
			
			//Start a RefreshToken thread, to get a new token.
			Thread refreshTokenThread = new Thread(new RefreshToken(id, refreshToken));
			refreshTokenThread.start();
		}
		
		App.logInfo("Completed reading storage file.");
	}
	
	public void writeStorage() {
		File storageFile = new File(configDirPath, "storage.json");
		
		try {
			BufferedWriter bw = new BufferedWriter(new FileWriter(storageFile));
			
			JSONArray storageContents = new JSONArray();
			for(User user : App.userMap.values()) {
				JSONObject json = new JSONObject();
				json.put("id", user.getId());
				json.put("email", user.getEmail());
				json.put("refreshToken", user.getRefreshToken());
				 
				storageContents.put(json);
			}
			
			JSONObject finalResult = new JSONObject();
			finalResult.put("storage", storageContents);
			
			bw.write(finalResult.toString());
			bw.flush();
			bw.close();
		} catch (IOException e) {
			App.logError("Unable to write to storage.json! Are your permissions set correctly?");
			App.logDebug(ExceptionUtils.getStackTrace(e));
			
			return;
		}
	}
}
