## Question 1 `Templates and Components`
A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?

## Answers
- [ ] Use and configure the teaser core component.
- [ ] Create a new custom component from scratch.
- [ ] Overlay the teaser core component.
- [x] Inherit from the teaser core component.

---

## Question 2 `OSGi Services`
A developer is working on a complex project with multiple bundles. One bundle provides an OSGi service for other bundles. Which two options are necessary to ensure that the other bundles can reference that OSGi service? (Choose two.)

## Answers
- [x] The bundles consuming the service need to import the fully qualified name of the service interface.
- [ ] The service needs to correctly declare metatype information.
- [ ] The bundle providing the service needs to contain a whitelist of allowed consumer bundles.
- [ ] The bundle providing the service needs to contain an adequate SCR descriptor file.
- [x] The bundle providing the service needs to export the java package of the service interface.

---

## Question 3 `Templates and Components`
The structure section of an editable template has a locked component. What happens to the content of that component when a developer unlocks it?

## Answers
- [ ] The content stays in the same place but it ignored on pages using the template.
- [x] The content is moved to the initial section of the editable template.
- [ ] The content is deleted after confirmation from the template author.
- [ ] The content is copied to the initial section of the editable template.

## [Reading](reading/question-3.md)

---

## Question 4 `Troubleshooting AEM projects`
Which log file contains AEM application request and response entries?

## Answers
- [ ] response.log
- [x] request.log
- [ ] history.log
- [ ] audit.log

## [Reading](http://www.sgaemsolutions.com/2017/04/aem-logs-in-detail-part-1.html)

---

## Question 5 `Templates and Components`
A developer wants to extend AEM Core Components to create a custom Carousel Component. How should the developer extend the Core Components?

## Answers
- [ ] Make changes to the original component and assign a component group.
- [x] Use the `sling:resourceSuperType` property to point to the core component.
- [ ] Use the `sling:resourceType` property to point to the core component.
- [ ] Copy the Core Carousel component to `/apps/<project>` folder.

## [Reading](reading/question-5.md)

---

## Question 6 `Troubleshooting AEM projects`
A developer wants to change the log level for a custom API. Which OSGi configuration should the developer modify?

## Answers
- [ ] Apache Sling Logging Configuration
- [ ] Apache Sling Log Tracker Service
- [x] Apache Sling Logging Writer Configuration
- [ ] Adobe Granite Log Analysis Service

## [Reading](reading/question-6.md)

---

## Question 7 `Installation and Configuration of AEM`
A developer is installing a content package with the package manager. The developer needs to restrict the approximate number of nodes in a batch that is saved to persistent storage in one transaction. How should the developer modify the number of transient nodes to be triggered until automatic saving?

## Answers
- [ ] AEM instances automatically modify the number of transient nodes based on the load balancing.
- [ ] Modify the export package manifest header and copy the content package to AEM installation folder.
- [ ] Select the option MergePreserve for the Access Control Handling drop-down in the Install Package dialog-box.
- [x] Change the value of Save Threshold in the Install Package dialog-box

---

## Question 8 `Templates and Components`
A developer creates Editable Templates based on a custom Page component. The developer wants to leverage the Style System within the Editable Templates to allow authors to switch between the Dark and Light Theme. The Style System dialog is NOT enabled for the site. What should the developer do to resolve this issue?:

## Answers
- [ ] Define Style Definitions using Page Policy dialog on Editable Template.
- [ ] Create two new client libraries with a dark and light theme and map them to the Page component.
- [x] Set the sling:resourceSuperType property to core/wcm/components/page/v2/page on the Page component.
- [ ] Create a new dialog for the custom Page components.

---

## Question 9 `OSGi Services`
A custom AEM application contains Bundle A and Bundle B. Bundle A has a dependency to Bundle B via Import-Package. How can both bundles be deployed most efficiently across all environments?

## Answers
- [ ] Use the Felix Web Console to upload the bundles in the correct order.
- [ ] Create one content package per bundle and use a package dependency to ensure installation order.
- [ ] Embed both bundles in one content package and use property `installationOrder` in package properties for correct bundle installation order.
- [x] Embed both bundles in one content package: the dependency via Import-Package is enough to ensure correct installation.

## [Reading](reading/question-9.md)

---

## Question 10 `Troubleshooting AEM projects`
After adding new features, a developer’s environment is experiencing slowness before ultimately running out of memory. The initial log analysis points towards a large number of open sessions. Which action should the developer take to further monitor the overall session count on this AEM instance?

## Answers
- [ ] Run the following command to generate thread dumps `jstack -l <pid> >> threaddumps.log`, analyze thread dumps to find long running sessions.
- [ ] Go to Web Console > Status > Threads, verify the overall thread count.
- [ ] Go to Tools > Operations > Monitoring. Create a new report based on Number of Active Sessions as metric.
- [x] Go to `<aem-install>/crx-qiuckstart/logs/strderr/log`, use the following command `grep -o‘CRXSessionImpl’ strderr.log| wc -l`

## [Reading](reading/question-10.md)

---

## Question 11 `Installation and Configuration of AEM`
An online insurance company website has user-generated content that must be replicated in all publish instances. What action should the developer take to achieve this?:

## Answers
- [ ] Configure the dispatcher flush agent in publish instance.
- [x] Configure reverse replication agents for the author.
- [ ] Configure the replication agent in the publish instances.
- [ ] Disable static agent in the author instance.

---

## Question 12 `OSGi Services`
A developer must create a workflow step that assigns a `WorkItem` to the appropriate person based on who has the least amount work to do. The group that must perform the action is configured into the workflow. Which non-deprecated interface should the Java implementation class use to perform the assignment?

## Answers
- [x] `com.adobe.granite.workflow.exec.ParticipantStepChooser`
- [ ] `com.day.cq.workflow.exec.ParticipantChooser`
- [ ] `com.day.cq.workflow.exec.WorkItem`
- [ ] `com.adobe.granite.workflow.exec.WorkflowData`

## [Reading](reading/question-12.md)

---

## Question 13 `Troubleshooting AEM projects`
From which AEM Web Console should a developer access and download full AEM Log Files?

## Answers
- [ ] Web Console -> System Information
- [x] Status -> Log files
- [ ] OSGI -> Sing Log Service
- [ ] AEM -> Log files

## [Reading](https://helpx.adobe.com/aem-forms/kb/getting-log-files-directly-from-aem.html)

---

## Question 14 `Templates and Components`
A developer needs to create a new component called “Component A”. Component A must show a list of other components that all have a resource type of existing “Component B”. Component A must render this list of tiles for each Component B where the tile rendering is different from the default one. The list of rendered tiles must be reusable by future new components. How should the developer implement this functionality?

## Answers
- [x] Create a script for tile rendering in Component B and use `data-sly-resource` attribute with a Sling selector in Component A to render the tile.
- [ ] Component A overlays Component B and overwrites the base renderer to facilitate the tiles.
- [ ] Component A inherits from Component B and overwrites the base renderer to facilitate the tiles.
- [ ] Component A calls the HTL of Component B directly using a `data-sly-include` attribute.

---

## Question 15 `Installation and Configuration of AEM`
For each CRX node in the hierarchy, which actions can be configured using the user admin interface?

## Answers
- [x] Read, Modify, Create, Delete, Read ACL, Edit ACL, Replicate
- [ ] Read, Write, Read ACL, Edit ACL, Replicate
- [ ] Read, Write, Delete, Edit ACL, Replicate
- [ ] Read, Modify, Create, Delete, Read ACL, Edit ACL

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/administering/security/security.html)

---

## Question 16 `OSGi Services`
An application runs specific license checks against certain DAM assets every day. It should send an email to a configured list if it finds warnings, and mark the asset accordingly. A service component that uses the Apache Sling Scheduler Service is created. DAM assets that must NOT be used anymore are hidden using ACLs and the license check must re-check them. How should a developer obtain a resource resolver that can read and update the DAM assets?

## Answers
- [ ] Set up a cron job with curl calls with the admin user and use `request.getResourse().getResourceResolver()`.
- [ ] Create a configuration line in PID `com.day.cq.security.ACLSetup` for the user that you obtain a session for via `ResourceResolverFactory.getResourceResolver(...)`.
- [ ] Configure the user admin in PID `org.apache.sling.serviceusermapping.impl.ServiceUserMapperImpl` as user.default and make sure the service user exists and has `jcr:read` and `jcr:write` permissions.
- [x] Create a configuration for PID `org.apache.sling.serviceusermapping.impl.ServiceUserMapperImpl.amended-damaccess` that references a pre-created service user with r/w permissions and use `ResourceResolverFactory.getServiceResourceResolver(...)`

---

## Question 17 `Templates and Components`
A developer is creating templates and/or components using CRXDE Lite. The developer needs to check the files into source control. Which tool should the developer use to achieve this goal?

## Answers
- [x] vlt command
- [ ] Content Explorer
- [ ] http://localhost:4502/crx/checkout
- [ ] mvn command

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/developing/devtools/ht-vlttool.html)

---

## Question 18 `OSGi Services`
A developer is creating a new OSGi bundle `com.custom.package.b` to expose new services. `com.custom.package.a` is already installed and active in the system and has the following package definition:
  ```
  Export-Package: com.custom.package.a;version="2.0"
  Import-Package: com.sample.package.a;version="[1,2]"
  Classpath: .,com.sample.package.b-3.0.jar
  ```
  The system console shows the following package availability:
  ```
  com.sample.package.a;version="1.5"
  com.sample.package.c;version="3.0"
  ```
  Bundle com.custom.package.b to be installed has the following package definition:
  ```
  Export-Package: com.custom.package.b;version="1.0"
  Import-Package: com.custom.package.a;version="[1,2)",com.sample.package.b;version="[3.0,3.0]",com.sample.package.c;version="[2,3)"
  ```
  What will happen when the developer uploads the bundle com.custom.package.b into the system?

## Answers
- [x] The bundle will install but fail the activation due to unsatisfied dependencies `com.sample.package.b` and `com.sample.package.c`.
- [ ] The bundle will install but fail the activation due to unsatisfied dependency `com.sample.package.c`.
- [ ] The bundle will install and activate successfully.
- [ ] The bundle will install but fail the activation due to unsatisfied dependency `com.sample.package.b`.

---

## Question 19 `OSGi Services`
A custom AEM application is using the PageManager API. What should a developer add to make the application compile and run correctly in AEM?

## Answers
- [x] a maven dependency to AEM uber-jar to the content package
- [ ] a maven dependency to bundle cq-wcm-core to the application bundle
- [ ] a maven dependency to AEM uber-jar to the application bundle
- [ ] a maven dependency to bundle cq-wcm-api to the content package

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/developing/devtools/ht-projects-maven.html)

---

## Question 20 `Installation and Configuration of AEM`
How should a developer enable remote debugging of an AEM server without modifying the AEM start script?

## Answers
- [ ] Enable the remote debugging service through the AEM Cloud Services menu.
- [ ] Rename the quickstart jar file to include the additional debug settings.
- [ ] Enable the remote debugging service through the AEM Web Console.
- [x] Include an additional JVM parameter when starting AEM with `java -jar`.

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/deploying/deploying/custom-standalone-install.html)

---

## Question 21 `Troubleshooting AEM projects`
A developer developed a workflow that makes a copy of every node created or modified under a certain path to a different one. The workflow launches but the nodes are not copied over. Which two methods should the developer use to resolve this issue? (Choose two.)

## Answers
- [x] Go to Workflow Failures screen and check if any instances of the workflow are present.
- [x] Go to Workflow instances screen and verify that the instance of the workflow is present and check its status.
- [ ] Go to Package Manager screen and reinstall the bundle that contains the workflow so it restarts.
- [ ] Go to Workflow Models screen, then delete and recreate the workflow.
- [ ] Go to Workflow Launchers and create a new launcher for the workflow even if one already exists.

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/administering/operations/workflows-administering.html)

---

## Question 21 `Templates and Components`
A developer creates an AEM editable template that includes a Layout Container. When the developer creates a page using this template, the Layout Container placeholder does NOT appear. What is causing this issue?

## Answers
- [ ] The Layout Container does NOT have a policy.
- [ ] The page template has NOT been enabled.
- [ ] The page template has NOT been published.
- [x] The Layout Container has NOT been unlocked.

---

## Question 22 `Troubleshooting AEM projects`
Too many pages are invalidated in the dispatcher cache when a page is published. What is most likely causing this issue in the dispatcher configuration?

## Answers
- [ ] Sticky session are NOT configured properly, resulting in requests being delivered to the wrong server.
- [x] The level of cache invalidation is NOT appropriate for the published content model.
- [ ] File globbing in the dispatcher configuration is NOT correct.
- [ ] The OS file system permissions are NOT properly configured.

## [Reading](https://experienceleaguecommunities.adobe.com/t5/adobe-experience-manager/entire-aem-dispacher-cache-invalid-after-every-page-publish/qaq-p/308697)

---

## Question 23 `OSGi Services`
A service component periodically retrieves content from an external REST interface and saves the information in JCR. The REST endpoint is configured via an OSGi service property. There is one URL for production (runmode prod) and another URL for all other environments. How should a developer configure the OSGi service?

## Answers
- [ ] Underneath `/apps/<project>/settings`, create the sub folders global and prod and node with name <PID>.conf each and configure the properties via node properties.
- [ ] Underneath `/config/<project>/settings`, create the sub folders config.default and config.prod and a file with the name <PID>.config each and list the properties as key value pairs in there.
- [x] Underneath `/apps/<project>`, create the sub folders config and config.prod and a file with the name <PID>.config each and list the properties as key value pairs in there.
- [ ] Underneath `/config/<project>/settings`, create the sub folders config and config.prod and a file with the name <PID>.config each and list the properties as key value pairs in there

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/deploying/configuring/configure-runmodes.html)

---

## Question 24 `Templates and Components`
What is the artifact ID of the maven dependency that contains all core AEM APIs?

## Answers
- [ ] core-jar
- [ ] api-jar
- [ ] aem-jar
- [x] uber-jar

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/developing/devtools/ht-projects-maven.html)

---

## Question 25 `Troubleshooting AEM projects`
There are performance, stability, and security issues with an installed AEM instance. What should a developer do to fix these issues?

## Answers
- [ ] Delete and reinstall the AEM instance.
- [ ] Install Adobe-provided Apache configuration file.
- [ ] Stop, clear cache files, and restart the AEM instance.
- [x] Install service pack updates from package share.

---

## Question 26 `Installation and Configuration of AEM`
A developer needs to ensure that the path `/content/<proj>/segments` exists on all environments with the correct initial content that the developer provides in a package. Content that exists in that path should NOT be affected. Which import mode should the developer use in the filter definition?

## Answers
- [ ] update
- [x] merge
- [ ] replace
- [ ] optional

---

## Question 27 `Installation and Configuration of AEM`
Which maven plugin is required to install a content package on a local AEM environment using maven?

## Answers
- [ ] Maven Install Plugin
- [ ] FileVault Package Maven Plugin
- [x] Content Package Maven Plugin
- [ ] Maven Bundle Plugin

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/developing/devtools/vlt-mavenplugin.html)

---

## Question 28 `Templates and Components`
A developer needs to implement a functionality that requires creating a Custom Workflow Step. Which two steps should the developer take to start developing the custom behavior? (Choose two.)

## Answers
- [ ] Implement a Java class with this method `public void process(WorkItem item, WorkflowSession wfsession) throws WorkflowException`.
- [ ] Implement a Java class extending from class com.adobe.granite.workflow.exec.WorkflowProcess.
- [x] Create a Workflow component node of the Super Resource Type `cq/workflow/components/model/process` under the folder `/apps/components`.
- [x] Implement a Java class implementing the interface `com.adobe.granite.workflow.exec.WorkflowProcess`.
- [ ] Create a Workflow component node of the Super Resource Type `cq/workflow/components/step` under the folder `/etc/workflow/models`.

## [Reading](reading/question-28.md)

---

## Question 29 `Installation and Configuration of AEM`
In which two ways can a developer keep simple and maintainable CRX Access Control Lists? (Choose two.)

## Answers
- [ ] Delete the ‘everyone’ group.
- [x] Use Deny statements sparingly.
- [x] Assign access rights to user groups rather than users.
- [ ] Assign access rights user by user.
- [ ] Use Deny statements extensively.

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/administering/security/security.html)

---

## Question 30 `Templates and Components`
A developer wants to create a Client Library that will only be included on touch enabled devices. What action should the developer take to achieve this?

## Answers
- [ ] Add the line “#base=touch” to the js.txt and css.txt files in the Client Library Folder.
- [ ] Create a resource folder called “touch” under the Client Library Folder.
- [x] Set the channels property on the Client Library Folder to “touch”.
- [ ] Pass the parameter user-agent=’touch’ when referencing the Client Library.

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/developing/introduction/clientlibs.html)

---

## Question 31 `Installation and Configuration of AEM`
A developer is working on a project locally and needs to install packages manually. The deployments to the localhost must be automated to speed up development. This functionality must be toggled on and off, depending on the needs of the developer. Which step should the developer take to achieve this?:

## Answers
- [ ] Configure the maven install plugin by defining the target URL, username, and password as maven properties.
- [ ] Run maven with the deploy phase. Maven will install the package on all local AEM instances running without further configuration.
- [x] Add a maven profile and configure the content package maven plugin within this profile.
- [ ] Write a script that does a PUT call to AEM each time maven builds a new package.

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/developing/devtools/vlt-mavenplugin.html)

---

## Question 32 `Installation and Configuration of AEM`
Which xml tag is used within the vault package definition to add a new path to a content package?

## Answers
- [x] `<filter>`
- [ ] `<content>`
- [ ] `<path>`
- [ ] `<rule>`

---

## Question 33 `Templates and Components`
Refer to the following Client Library node structure:
  ```
  + clientlibs
    - jcr:primaryType="cq:ClientLibraryFolder"
    - categories="[clientlibs.example]"
    + js.txt
      - jcr:primaryType="nt:file"
    + css.txt
      - jcr:primaryType="nt:file"
    + js
      - jcr:primaryType="nt:folder"
      + javascript1.js
      + javascript2.js
  ```
  The js.txt looks like:
  ```
  + javascript1.js
  + javascript2.js
  ```
  The JavaScript is NOT present in the Client Library when it is loaded. What should a developer do to resolve this issue?

## Answers
- [x] Add `#base=js` as the first line in the js.txt file.
- [ ] Change the js folder to a Client Library node and add the property `categories="[clientlibs.example]"`.
- [ ] Split the js and css into 2 Client Libraries since they can’t be in the same Client Library.
- [ ] Change the js folder to a Client Library node and embed it on the clientlibs node.

---

## Question 34 `Installation and Configuration of AEM`
A developer wants to automatically truncate request log files once they exceed 10 MB. Which OSGi configuration should the developer modify?

## Answers
- [ ] Apache Sling Customizable Request Data Logger
- [ ] Adobe Granite Log Analysis Service
- [x] Apache Sling Logging Writer Configuration
- [ ] Apache Sling Logging Configuration

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/deploying/configuring/osgi-configuration-settings.html)

---

## Question 35 `Templates and Components`
A developer has a component located under the path /apps. This component has a Client Library which is directly loaded onto a page. The publish instance loads the page correctly. The dispatcher also loads the page but the Client Library is missing. How should the developer resolve this issue, while taking security into consideration?

## Answers
- [ ] Change the ACLs for the Client Library.
- [ ] Move the Client Library under `/apps/<project>` library.
- [x] Add the property `allowProxy` with a boolean value true.
- [ ] Allow the path to the clientlibs on the dispatcher.

## [Reading](./reading/question-35.md)

---

## Question 36 `Templates and Components`
A Client-Side Library has the category “library.example”. Which HTL statement should a developer use to reference only the CSS files of this Client-Side Library?

## Answers
- [x] `<sly data-sly-use.clientlib=”/libs/granite/sightly/templates/clientlib.html” data-sly-call=”${clientlib.css @ categories=’library.example’}”/>`
- [ ] `<sly data-sly-use.clientlib=”/libs/granite/sightly/templates/clientlib.html” data-sly-call=”${clientlib.css @ category=’library.example’}”/>`
- [ ] `<sly data-sly-use.clientlib=”/libs/granite/sightly/templates/clientlib.html” data-sly-call=”${clientlib.all @ categories=’library.example.css’}”/>`
- [ ] `<sly data-sly-use.clientlib=”/libs/granite/sightly/templates/clientlib.html” data-sly-call=”${clientlib.all @ type=’css’, categories=’library.example’}”/>`

---

## Question 37 `Templates and Components`
A developer is creating a custom component on the page `/latestBlogs.html` that needs to list all the titles of the blogs pages under `/content/blogs`. How does this component get the list of child pages?

## Answers
- [ ] Instantiate a node object with `session.getNode("/content/blogs")` and then iterate through the child nodes and print the title for each.
- [ ] Use `PageManager.getPage(“/content/blogs”)` of the static `PageManager` class to instantiate a Page object and then iterate through the child pages and print the title for each.
- [ ] Use the `QueryDebugger` to look for all children of `/content/blogs` and then iterate through the result set and print the title for each.
- [x] Adapt the `resourceResolver` to the `PageManager` service, then use the `getPage("/content/blogs")` to instantiate a Page object and then iterate through the child pages and print the title for each.

---

## Question 38 `OSGi Services`
A banking AEM application contains functionality to calculate a mortgage rate based on user input. A servlet in place calculates the result in the backend. A call to an internal third-party REST service is required to retrieve the average object value based on a given zip code. The following three service interfaces are used: `MortgageCalculationServlet`, `MortgageCalculationService` and `ObjectValueLookupRestService` where `MortgageCalculationServlet` has a dependency to `MortgageCalculationService` and `MortgageCalculationService` has a dependency to `ObjectValueLookupRestService`. The calculation has many combinations of input parameters and edge cases, so the JUnit coverage must be as high as possible. Which two strategies should the developer use to ensure testability of the application code? (Choose two.)

## Answers
- [ ] Use `BundleContext.getServiceReference(...)` and `BundleContext.getService(...)` in application code to look up the required services just before usage.
- [ ] Use static methods to avoid boilerplate in application code.
- [x] Use a mock framework to be able to create and inject mocks in the test code.
- [x] Use the standard OSGi @Reference annotation to wire the dependencies in application code.
- [ ] Deploy a third party dependency injection container to wire dependencies more efficiently in application code.

---

## Question 39 `Templates and Components`
A developer creates an editable template with a Layout Container. The developer needs to restrict the Layout Container to just one column layout. What should the developer do to the editable template to enforce this restriction?:

## Answers
- [x] Using Template Editor, set the responsive settings to 1 column for Layout Container Policy.
- [ ] Add responsive column control component to the template and set column type to 1.
- [ ] Overlay wcm.foundation.components.page.responsive Client Library and set @max_col to 1.
- [ ] Using Template Editor, lock the Structure Component for the template.

---

## Question 40 `OSGi Services`
A custom AEM application has a run time dependency to a third party OSGi bundle that is NOT included in out-of-the-box AEM. The third party dependency needs to be available for multiple applications and be upgraded separately from the custom AEM application. How should a developer make sure that the bundle is installed on all environments?:

## Answers
- [ ] Add the dependency to the third party bundle in the pom.xml of the project bundle.
- [ ] Embed the third party bundle in the bundle that depends on it.
- [ ] Embed the bundle in a content package to have it automatically deployed.
- [x] Declare the dependency correctly using a link to the OSGi Bundle Repository (OBR).

---

## Question 41 `Templates and Components`
A developer is working with the following HTL expression in a component rendering script:
  ```
  ${'path/page.infinity.json' @ extension = 'html',
    removeSelectors = ['foo'],
    selectors = ['foo', 'bar'],
    prependSuffix = 'hello',
    suffix = 'world'
  }
  ```
  What is the expected output of this expression?:

## Answers
- [ ] path/page.bar.html/world
- [x] path/page.bar.html/hello/world
- [ ] path/page.foo.bar.html/hello/world
- [ ] path/page.infinity.json.bar.html/world

---

## Question 42 `Installation and Configuration of AEM`
A developer running a local AEM instance and working on an AEM project needs to change a large number of files locally in the filesystem. The developer needs to get the changes uploaded to the local AEM instance to verify changes almost immediately in the browser. What action should the developer take to most efficiency meet these requirements?

## Answers
- [ ] Access CRXDE and upload the files through the interface.
- [ ] Make the changes in CRXDE create a content package, download it, and expand it into the working directory after each change.
- [x] Install FileVault bundle in the AEM instance and register the local working directory for synchronization.
- [ ] Build a Content Package using maven and deploy it after each change.

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-64/developing/devtools/ht-vlttool.html)

---

## Question 43 `Templates and Components`
A developer wants to consume AEM Page Data in a Single Page Application. The Single Page Application is coded to understand JSON format. Only page content should be exposed through JSON. All the existing components are based on foundation components. Which change should the developer make in the existing components to support this requirement?

## Answers
- [ ] Add JSON as the default extension in Apache Sling Servlet/Split Resolver and Error Handler Configuration.
- [ ] Invoke the page URL with the extension .json to get the values to construct the required output.
- [x] Implement a Sling Model Exporter for the components.
- [ ] Create a custom sling event handler to handler JSON requests.

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/developing/components/json-exporter-components.html)

---

## Question 44 `OSGi Services`
A custom AEM application contains a service component that needs to access the JCR repository within the activate method. The activate method uses `ResourceResolverFactory.getServiceResourceResolver(...)` without specifying a sub service name. What should a developer do to make sure the user service mapping for the service component is available?

## Answers
- [x] Create a field of type ServiceUserMapped and annotate it with `@Reference`.
- [ ] Wait for the service ServiceUserMapper via `BundleContext.getServiceReference(...)`.
- [ ] Create a field of type ServiceUserMapped and annotate it with `@Reference` using `ReferencePolicy.DYNAMIC`.
- [ ] Create a field of type ServiceUserMapper and annotate it with `@Reference` using `ReferencePolicy.STATIC`.

## [Reading](https://taradevko.com/aem/aem-service-user-mapper-do-you-know-it/)

---

## Question 45 `Troubleshooting AEM projects`
After a recent code deployment, an AEM site is experiencing longer than usual query execution time. The deployment package contained some new Lucene index definitions. A developer needs to identify the long running queries and confirm that the new index definitions are getting applied correctly. Which action should the developer take to investigate this problem?

## Answers
- [ ] Go to Tools > Operations >Diagnosis > Download Thread Dumps. Analyze the Thread Dumps to identify long running requests.
- [ ] Go to Tools > Operations >Diagnosis > Log Message. Configure DEBUG log level on com.day.cq.search to monitor search queries.
- [ ] Go to Tools > Operations > Diagnosis > Index Manager. Select the new Indexes and run a consistency check.
- [x] Go to Tools > Operations >Diagnosis > Query Performance > Slow Queries. Select a Query and Click on Explain.

---

## Question 46 `Installation and Configuration of AEM`
A developer installs the latest Service pack to a local AEM author instance. How should the developer install this package on the publish instance?

## Answers
- [ ] Replicate from package manager of publish instance.
- [ ] Use upload/install from OSGI console of publish instance.
- [ ] Use upload/install from OSGI console of author instance.
- [x] Replicate from package manager of author instance.

---

## Question 47 `Installation and Configuration of AEM`
Two AEM publish feed a single Dispatcher. Which part of the Dispatcher configuration should a developer review to ensure both AEM publish instance are used?([[]]):

## Answers
- [ ] virtualhosts
- [ ] cache
- [ ] filter
- [x] farms

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-dispatcher/using/configuring/dispatcher-configuration.html#defining-farms-farms)

---

## Question 48 `Templates and Components`
74. A developer needs an existing workflow to run only when pages are created under a certain folder. What should the developer create to achieve this?

## Answers
- [ ] A Launcher with the field exclude that has the value !jcr:nodetype==cq:Page.
- [x] A Launcher with the field path pointing to the folder and nodetype field have the value cq:Page.
- [ ] A Launcher with the field path pointing to the folder and condition field and have the value jcr:content/jcr:primaryType==cq:Page.
- [ ] A Launcher with the field condition that has the value jcr:content/page.

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/administering/operations/workflows-starting.html)

---

## Question 49 `Templates and Components`
A developer creates a template-type for building editable templates. The resulting editable templates and pages must always contain a specific layout container that can NOT be deleted by the author. How should the developer meet this requirement?

## Answers
- [ ] Add the layout container component by including it on the actual page component.
- [ ] Add a content policy to the template-type to disable the removal of the layout container.
- [ ] Add the layout container component to the initial section of the template-type.
- [x] Add the layout container component to the structure section of the template-type.

---

## Question 50 `Templates and Components`
A developer creates two custom classes. ClassA has the following code:
  ```java
    package com.aem.abc;
    import org.slf4j.Logger;
    import org.slf4j.LoggerFactory;
    public class ClassA {
      private static final Logger logger = LoggerFactory.getLogger(this.getClass());
      public void classMethod() {
        logger.debug("Message from ClassA method");
      }
    }
  ```
The developer creates a custom log custom.log with debug level in OSGi sling log support for the Java package com.aem.abc. The developer adds another class ClassB with the following code:
  ```java
    package com.aem.xyz;
    import org.slf4j.Logger;
    import org.slf4j.LoggerFactory;
    public class ClassB {
      private static final Logger logger = LoggerFactory.getLogger(this.getClass());
      public void classMethod() {
        logger.debug("Message from ClassB method");
      }
    }
  ```
  Which action must the developer take to see the log message in the same file from both classes?

## Answers
- [ ] Create separate a log file in the OSGi web console ->Sling -> Log Support for logger com.aem.xyz.
- [ ] Configure custom.log in the OSGi web console -> Sling -> Log Support and replace com.aem.xyz with com.aem.abc.
- [x] Configure custom.log in the OSGi web console -> Sling -> Log Support and replace logger com.aem.abc with com.aem.
- [ ] Configure custom.log in the OSGi web console -> Sling -> Log Support and replace the package com.aem.abc with com.aem.xyz.

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/deploying/configuring/monitoring-and-maintaining.html#working-with-audit-records-and-log-files)

---

## Question 51 `Installation and Configuration of AEM`
Which environment variable in the AEM start script should a developer modify to enable remote debugging?

## Answers
- [ ] CQ_HOST
- [x] CQ_JVM_OPTS
- [ ] CQ_PORT
- [ ] CQ_RUNMODE

---

## Question 52 `Installation and Configuration of AEM`
AEM is installed in $AEM_HOME. In which subfolder are the command line startup and shutdown scripts located?

## Answers
- [x] $AEM_HOME/crx-quickstart/bin/
- [ ] $AEM_HOME/crx-quickstart/scripts/
- [ ] $AEM_HOME/
- [ ] $AEM_HOME/crx-quickstart/opt/

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-65/deploying/deploying/deploy.html)

---

## Question 53 `Installation and Configuration of AEM`
A customer bundle of an application is in state “Installed” after deploying it with Maven. What should a developer do to change it to state “Active”?:

## Answers
- [ ] Use the “Update” action for the bundle in the Apache Felix Web Console.
- [ ] Use the “Start” action for the bundle in the Apache Felix Web Console.
- [ ] Reinstall the content package using the package manager.
- [x] Ensure all OSGi requirements are met and re-deploy using Maven.

---

## Question 54 `Installation and Configuration of AEM`
How should a developer configure the replication agent to flush the dispatcher cache for a newly activated page?([[]]):

## Answers
- [ ] Set the serialization type property of the default agent to dispatcher flush.
- [ ] Create a new replication agent and set transport URI to point to the dispatcher.
- [x] Create a dispatcher flush agent in publish instance
- [ ] Create a reserve replication agent on the author instance.

## [Reading](https://docs.adobe.com/content/help/en/experience-manager-dispatcher/using/configuring/page-invalidate.html#invalidating-dispatcher-cache-from-a-publishing-instance)

---

## Question 55 `Troubleshooting AEM projects`
Which log file should a developer use to search for exception stack traces?

## Answers
- [ ] <aem-install>/crx-quickstart/logs/request.log
- [x] <aem-install>/crx-quickstart/logs/error.log
- [ ] <aem-install>/crx-quickstart/logs/access.log
- [ ] <aem-install>/crx-quickstart/logs/info.log

---

## Question 56 `Installation and Configuration of AEM`
A developer needs to configure sets of values according to the following parameters:
  - Varies for different staging environments
  - Varies for different content paths
  - Differs between author and publish
  Which implementation strategy should the developer use to meet these requirements?

## Answers
- [ ] A custom cloud configuration
- [x] A context aware configuration with buckets using an OSGi configuration
- [ ] One OSGi configuration for the set of values with runmodes
- [ ] A JCR property at the content root node of the site with inheritedPageProperties

## [Reading](https://sling.apache.org/documentation/bundles/context-aware-configuration/context-aware-configuration.html)

---

## Question 57 `Templates and Components`
A developer needs to upgrade existing components (Proxy Components) based on Core Components Version 1(v1) to Core Components Version 2(v2). How should the developer upgrade to V2 Core Components?

## Answers
- [x] Modify the sling:resourceSuperType property on the proxy component to point to V2 Component.
- [ ] Modify the sling:resourceSuperType property on the proxy component to point to V1 Component.
- [ ] Create a new Proxy Component and set sling:resourceType property to V2 Core Component.
- [ ] Proxy Components will be automatically upgraded to the V2 Core Component on AEM Restart.

---

## Question 58 `Installation and Configuration of AEM`
In which maven build phase is the content package assembled?

## Answers
- [ ] compile
- [ ] deploy
- [ ] package
- [x] install

---
