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

